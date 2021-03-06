use core::panic;
use ethers::{abi::Token, core::abi::Contract as Abi, prelude::*};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use std::sync::Arc;

use crate::utils::{fs::*, parse::*};

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
        let contract_vec = contract.split(":").collect::<Vec<&str>>();
        if contract_vec.len() != 2 {
            panic!("Invalid contract format: {}", contract);
        }

        let sol_file = contract_vec[0];
        if !Path::new(sol_file).exists() || !sol_file.ends_with(".sol") {
            println!(
                "Contract {} not exists or isn't sol file, pls check, sweet~~~",
                contract
            );
        }

        ContractInfo {
            name: contract_vec[1].to_string(),
            contract: sol_file.to_string(),
            args,
            abi: Abi::default(),
            bytecode: Bytes::default(),
        }
    }

    pub async fn compile(&mut self) -> Result<(), io::Error> {
        match is_contract_existed(self.contract.clone()) && self.contract.ends_with(".sol") {
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
                println!("Contract {} not found", self.contract);
                Ok(())
            }
        }
    }

    pub async fn run<M: Middleware + 'static>(&mut self, provider: M) -> eyre::Result<()> {
        // compile to get abi and bytecode
        self.compile().await?;
        let abi = self.abi.clone();
        let bin = self.bytecode.clone();

        // Add arguments to constructor
        let args = parse_constructor_args(&abi.clone().constructor.unwrap(), &self.args)?;

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
        let provider = Arc::new(provider);
        let factory = ContractFactory::new(abi, bin, provider.clone());

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
        println!("Transaction hash: {:?}\n", receipt.transaction_hash);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers::utils::Anvil;
    use std::{
        future::Future,
        time::{Duration, SystemTime},
    };

    pub async fn run_at_least_duration(duration: Duration, block: impl Future) {
        let start = SystemTime::now();
        block.await;
        if let Some(sleep) = duration.checked_sub(start.elapsed().unwrap()) {
            tokio::time::sleep(sleep).await;
        }
    }

    #[tokio::test]
    async fn test_deploy_success() {
        // given
        let mut contract_info = ContractInfo {
            name: "SimpleStorage".to_string(),
            contract: "examples/contract.sol".to_string(),
            args: vec!["value".into()],
            abi: Abi::default(),
            bytecode: Bytes::default(),
        };

        // use dev env here
        // let client = get_provider(&anvil, dotenv!("RPC_URL").to_string(), dotenv!("PRI_KEY").to_string()).await;

        // need declare here to guarantee anvil's lifetime
        let anvil = &Anvil::new().spawn();
        // use anvil endpoint here
        let client = get_provider(&anvil, "".to_string(), "".to_string()).await;

        // when
        run_at_least_duration(Duration::from_millis(250), async {
            contract_info.run(client).await.unwrap();
        })
        .await
    }
}
