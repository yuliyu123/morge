use std::sync::Arc;

use crate::config::Config;
use crate::contract::ContractInfo;
use crate::utils::*;
use ethers::core::abi::Contract as Abi;
use ethers::prelude::*;

pub struct Executer {
    pub cfg: Config,
    // verifier: Verifier,
}

impl Executer {
    pub fn new() -> Self {
        Self {
            cfg: Config {
                rpc_url: None,
                pri_key: None,
                contracts: vec![],
            }, // verifier: None,
        }
    }

    pub fn set_config(&mut self, cfg: Config) {
        self.cfg = cfg;
    }

    pub async fn run(self) -> eyre::Result<()> {
        match !self.cfg.contracts.is_empty() {
            true => {
                let provider = get_http_provider(
                    &self
                        .cfg
                        .rpc_url
                        .unwrap_or_else(|| "http://localhost:8545".to_string()),
                    false,
                );

                let chain_id = provider.get_chainid().await?;
                let wallet =
                    get_from_private_key(&self.cfg.pri_key.unwrap_or_else(|| "".to_string()));
                let wallet = wallet?.with_chain_id(chain_id.as_u64());
                let provider = SignerMiddleware::new(provider.clone(), wallet);
                let provider = Arc::new(provider);

                for mut contract in self.cfg.contracts {
                    contract.run(provider.clone()).await?;
                }
                return Ok(());
            }
            false => return Err(eyre::eyre!("no contract to run")),
        }
    }
}

#[tokio::test]
async fn test_batch_run_success() {
    let contract_info = ContractInfo {
        name: "SimpleStorage".to_string(),
        contract: "examples/contract.sol".to_string(),
        args: vec!["value".into()],
        abi: Abi::default(),
        bytecode: Bytes::default(),
    };

    let cfg = Config {
        rpc_url: Some(dotenv!("RPC_URL").to_string()),
        pri_key: Some(dotenv!("PRI_KEY").to_string()),
        contracts: [contract_info].to_vec(),
    };

    println!("cfg: {:?}", cfg);

    let mut executer = Executer::new();
    executer.set_config(cfg);
    executer.run().await.unwrap();
}
