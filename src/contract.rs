// pub mod utils;
use crate::utils::*;
use ethers::core::abi::Contract as Abi;
use ethers::prelude::artifacts::bytecode;
use ethers::{
    abi::{Constructor, Token},
    prelude::ContractFactory,
};
use ethers::{prelude::*, utils::Anvil};
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{convert::TryFrom, sync::Arc, time::Duration};
use std::{io, option};

extern crate dotenv;

use dotenv::dotenv;
use std::env;


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
            name: name,
            contract: contract,
            args: args,
            abi: Abi::default(),
            bytecode: Bytes::default(),
        }
    }
    pub async fn compile(&mut self) -> Result<(), io::Error> {
        let contract = Path::new(&env!("CARGO_MANIFEST_DIR")).join(&self.contract);
        if !contract.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "contract path not found",
            ));
        }

        let compiled = Solc::default()
            .compile_source(contract)
            .expect("Could not compile contracts");
        let (abi, bytecode, _runtime_bytecode) = compiled
            .find(self.name.to_string())
            .expect("could not find contract name")
            .into_parts_or_default();
        self.abi = abi;
        self.bytecode = bytecode;
        Ok(())
    }

    // pub async fn create_and_run(&self, contract_info: &mut ContractInfo, provider, signer) -> eyre::Result<()> {
    pub async fn run<M: Middleware + 'static>(&mut self, provider: M) -> eyre::Result<()> {
        // compile to get abi and bytecode
        self.compile().await?;
        let abi = self.abi.clone();
        let bin = self.bytecode.clone();

        // Add arguments to constructor
        let args = self.parse_constructor_args(&abi.clone().constructor.unwrap(), &self.args)?;

        // deploy contract
        self.deploy(abi.clone(), bin.clone(), args, provider).await?;
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
        let factory = ContractFactory::new(abi.clone(), bin, provider.clone());

        // start deploy
        let is_args_empty = self.args.is_empty();
        let deployer = factory.deploy_tokens(args.clone()).context("Failed to deploy contract").map_err(|e| {
            if is_args_empty {
                e.wrap_err("No arguments provided for contract constructor. Consider --constructor-args")
            } else {
                e
            }
        })?;

        let deployer_address = provider
            .default_sender()
            .expect("no sender address set for provider");
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

#[cfg(test)]
mod tests {
    use super::*;

    // fn get_provider<M: Middleware + 'static>() -> Arc<M> {
    //     println!("RPC_URL: {}", dotenv!("RPC_URL"));
    //     println!("PRI_KEY: {}", dotenv!("PRI_KEY"));
    //     let provider = get_http_provider(
    //         dotenv!("RPC_URL"),
    //         false,
    //     );
    //     let wallet = get_from_private_key(dotenv!("PRI_KEY"));
    //     let provider = SignerMiddleware::new(provider.clone(), wallet.unwrap());
    //     Arc::new(provider)
    // }

    // // 0x5fbd…0aa3
    #[tokio::test]
    async fn test_deploy() {
        let mut contract_info = ContractInfo {
            name: "SimpleStorage".to_string(),
            contract: "src/examples/contract.sol".to_string(),
            args: vec![],
            abi: Abi::default(),
            bytecode: Bytes::default(),
        };
        // let provider = get_provider();
        // super::run(&mut contract_info, provider).await;
    }
}
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
