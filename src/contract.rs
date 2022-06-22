use crate::utils::*;
use ethers::{
    abi::{Constructor, Token},
    core::abi::Contract as Abi,
    prelude::*,
};
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use std::sync::Arc;

// contract info to deploy
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractInfo {
    pub name: String,
    pub contract: String,
    pub args: Vec<String>,
    pub abi: Abi,
    pub bytecode: Bytes,
}

impl ContractInfo {
    pub fn new(contract: String, args: Vec<String>) -> Self {
        if !Path::new(&contract.split(":").collect::<Vec<&str>>()[0]).exists() {
            panic!("contract {} not exists", contract);
        }
        let contract_vec = contract.split("/").collect::<Vec<&str>>();

        let mut name = "".to_string();
        match contract_vec.len() {
            1 => {
                name = contract_vec[0].to_string();
            }
            _ => {
                name = contract_vec[contract_vec.len() - 1].to_string();
            }
        }
        ContractInfo {
            name,
            contract,
            args,
            abi: Abi::default(),
            bytecode: Bytes::default(),
        }
    }
    
    pub async fn compile(&mut self) -> Result<(), io::Error> {
        match is_contract_existed(self.contract.clone()) {
            true => {
                let compiled = Solc::default()
                    .compile_source(self.contract.clone())
                    .expect("Could not compile contracts");
                let (abi, bytecode, _runtime_bytecode) = compiled
                    .find(self.name.to_string())
                    .expect("could not find contract name")
                    .into_parts_or_default();
                self.abi = abi;
                self.bytecode = bytecode;
                Ok(())
            }
            false => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "contract path not found",
                ));
            }
        }
    }

    // pub async fn create_and_run(&self, contract_info: &mut ContractInfo, provider, signer) -> eyre::Result<()> {
    pub async fn run<M: Middleware + 'static>(&mut self, provider: M) -> eyre::Result<()> {
        // compile to get abi and bytecode
        self.compile().await?;
        let abi = self.abi.clone();
        let bin = self.bytecode.clone();

        // Add arguments to constructor
        let args = parse_constructor_args(&abi.clone().constructor.unwrap(), &self.args)?;

        println!("args: {:?}", args);
        // deploy contract
        self.deploy(abi.clone(), bin.clone(), args, provider)
            .await?;
        Ok(())
    }

    pub async fn deploy<M: Middleware + 'static>(
        &self,
        abi: Abi,
        bin: Bytes,
        args: Vec<Token>,
        provider: M,
    ) -> eyre::Result<()> {
        println!("deploying contract abi: {:?}", abi);
        let provider = Arc::new(provider);
        let factory = ContractFactory::new(abi, bin, provider.clone());

        println!("contract args {:?}", args);
        // start deploy
        let deployer = factory.deploy_tokens(args.clone())?.legacy();

        let deployer_address = provider
            .default_sender()
            .expect("no sender address set for provider");
        println!("Deployer address: {}", deployer_address);
        let (deployed_contract, receipt) = deployer.send_with_receipt().await?;
        let address = deployed_contract.address();

        println!("Deployer: {deployer_address:?}");
        println!("Deployed to: {:?}", address);
        println!("Transaction hash: {:?}", receipt.transaction_hash);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ethers::utils::{Anvil, AnvilInstance};

    use super::*;

    pub fn connect(anvil: &AnvilInstance, idx: usize) -> Arc<Provider<Http>> {
        let sender = anvil.addresses()[idx];
        let provider = Provider::<Http>::try_from(anvil.endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64))
            .with_sender(sender);
        Arc::new(provider)
    }

    // // 0x5fbd…0aa3
    #[tokio::test]
    async fn test_deploy_success() {
        // given
        let mut contract_info = ContractInfo {
            name: "SimpleStorage".to_string(),
            contract: "src/examples/contract.sol".to_string(),
            args: vec!["value".into()],
            abi: Abi::default(),
            bytecode: Bytes::default(),
        };

        let provider = get_http_provider(dotenv!("RPC_URL"), false);
        let chain_id = provider.get_chainid().await.unwrap();
        let wallet = get_from_private_key(dotenv!("PRI_KEY"));
        let wallet = wallet.unwrap().with_chain_id(chain_id.as_u64());
        let provider = SignerMiddleware::new(provider.clone(), wallet);
        let provider = Arc::new(provider);

        // when
        contract_info.run(provider).await.unwrap();
    }

    #[tokio::test]
    async fn test_anvil_deploy_success() {
        let mut contract_info = ContractInfo {
            name: "SimpleStorage".to_string(),
            contract: "src/examples/contract.sol".to_string(),
            args: vec!["value".into()],
            abi: Abi::default(),
            bytecode: Bytes::default(),
        };
        contract_info.compile().await.unwrap();
        let anvil = Anvil::new().spawn();

        let client = connect(&anvil, 0);
        let factory = ContractFactory::new(
            contract_info.abi.clone(),
            contract_info.bytecode.clone(),
            client.clone(),
        );
        let abi = contract_info.abi.clone();
        let args =
            parse_constructor_args(&abi.clone().constructor.unwrap(), &contract_info.args).unwrap();
        println!("args: {:?}", args);

        let deployer = factory.deploy_tokens(args).unwrap().legacy();
        // assert!(deployer.call().await.is_ok());
        let (contract, receipt) = deployer.send_with_receipt().await.unwrap();
        assert_eq!(receipt.contract_address.unwrap(), contract.address());
        println!("receipt: {}", receipt.contract_address.unwrap());

        let get_value = contract.method::<_, String>("getValue", ()).unwrap();
        println!("get_value: {:?}", get_value.call().await.unwrap());
    }
}
