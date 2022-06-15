// pub mod utils;
use ethers::core::abi::Contract as Abi;
use ethers::{prelude::*, utils::Anvil};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use std::{convert::TryFrom, sync::Arc, time::Duration};
use eyre::{Result, Context};
use ethers::{
    abi::{Constructor, Token},
    prelude::{ContractFactory},
};
use crate::utils::*;

// contract info to deploy
#[derive(Serialize, Deserialize)]
pub struct ContractInfo {
    pub name: String,
    pub path: String,
    pub args: Vec<String>,
    pub abi: Abi,
    pub bytecode: Bytes,
}

impl ContractInfo {
    pub fn new(path: String, args: Vec<String>) -> Self {
        if args.is_empty() {
            return ContractInfo {
                name: path,
                path: "".to_string(),
                args: vec![],
                abi: Abi::default(),
                bytecode: None.unwrap(),
            };
        }
        ContractInfo {
            name: path,
            path: "".to_string(),
            args: vec![],
            abi: Abi::default(),
            bytecode: None.unwrap(),
        }
    }
    pub async fn compile(&self, contract_info: &mut ContractInfo) -> Result<(), io::Error> {
        let contract_path = Path::new(&env!("CARGO_MANIFEST_DIR")).join(&contract_info.path);
        if !contract_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "contract path not found"));
        }
        
        let compiled = Solc::default().compile_source(contract_path).expect("Could not compile contracts");
        let (abi, bytecode, _runtime_bytecode) =
        compiled.find(contract_info.name.to_string()).expect("could not find contract name").into_parts_or_default();
        contract_info.abi = abi;
        contract_info.bytecode = bytecode;
        Ok(())
    }

    // pub async fn create_and_run(&self, contract_info: &mut ContractInfo, provider, signer) -> eyre::Result<()> {
    pub async fn run(&self, contract_info: &mut ContractInfo) -> eyre::Result<()> {
        // compile to get abi and bytecode
        self.compile(contract_info).await?;
        let abi = contract_info.abi.clone();
        let bin = contract_info.bytecode.clone();

        // Add arguments to constructor
        let args = self.parse_constructor_args(&abi.clone().constructor.unwrap(), &self.args)?;

        // deploy contract
        self.deploy(abi.clone(), bin.clone(), args).await?;
        Ok(())
    }

    pub async fn deploy(
        &self,
        abi: Abi,
        bin: Bytes,
        args: Vec<Token>
    ) -> eyre::Result<()> {
        let anvil = Anvil::new().spawn();
        let wallet: LocalWallet = anvil.keys()[0].clone().into();
        let provider =
            Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));
        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = Arc::new(client);
        let factory = ContractFactory::new(abi.clone(), bin, client.clone());

        // start deploy
        let is_args_empty = self.args.is_empty();
        let deployer = factory.deploy_tokens(args.clone()).context("Failed to deploy contract").map_err(|e| {
            if is_args_empty {
                e.wrap_err("No arguments provided for contract constructor. Consider --constructor-args")
            } else {
                e
            }
        })?;

        let deployer_address =
            provider.default_sender().expect("no sender address set for provider");
        let (deployed_contract, receipt) = deployer.send_with_receipt().await?;
        let address = deployed_contract.address();

        println!("Deployer: {deployer_address:?}");
        println!("Deployed to: {:?}", address);
        println!("Transaction hash: {:?}", receipt.transaction_hash);

        Ok(())
    }

    fn parse_constructor_args(
        &self,
        constructor: &Constructor,
        constructor_args: &[String],
    ) -> Result<Vec<Token>> {
        let params = constructor
            .inputs
            .iter()
            .zip(constructor_args)
            .map(|(input, arg)| (&input.kind, arg.as_str()))
            .collect::<Vec<_>>();

        parse_tokens(params, true)
    }
}

// // 0x5fbd…0aa3
// #[tokio::test]
// async fn test_deploy() {
//     let mut contract_info = ContractInfo {
//         name: "SimpleStorage".to_string(),
//         path: Some("src/examples/contract.sol".to_string()),
//         source: None,
//         args: vec![],
//         abi: Abi::default(),
//         bytecode: Bytes::default(),
//     };
//     deploy(&mut contract_info).await;
// }


// #[tokio::test]
// async fn test_compile_success() {
//     let mut contract_info = ContractInfo {
//         name: "SimpleStorage".into(),
//         path: Some("src/examples/contract.sol".into()),
//         source: None,
//         args: vec![],
//         abi: Abi::default(),
//         bytecode: Bytes::default(),
//     };
//     compile(&mut contract_info).await;
//     println!("{}: ", serde_json::to_string(&contract_info).unwrap());
// }


// #[tokio::test]
// #[ignore = "skipped"]
// async fn test_compile_with_noexisted_file() {
//     let mut contract_info = ContractInfo {
//         name: "SimpleStorage".to_string(),
//         path: Some("examples/contract.sol".to_string()),
//         source: None,
//         args: vec![],
//         abi: Abi::default(),
//         bytecode: Bytes::default(),
//     };
//     let _err = compile(&mut contract_info).await;
//     println!("{}: ", serde_json::to_string(&contract_info).unwrap());
//     // assert_eq!(err.err(), io::Error::new(io::ErrorKind::NotFound, "contract path not found"));
// }
