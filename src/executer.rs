use std::sync::Arc;

use crate::config::{restore_cfg, save, Config};
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

    pub fn init() -> eyre::Result<()> {
        save(&Config::new())?;
        Ok(())
    }

    pub fn set_rpc_and_key(rpc_url: &str, pri_key: &str) -> eyre::Result<()> {
        let mut cfg = restore_cfg()?;
        cfg.set_rpc_and_key(rpc_url.to_string(), pri_key.to_string())?;
        Ok(())
    }

    fn set_config(&mut self, cfg: Config) {
        self.cfg = cfg;
    }

    pub fn add_contract(contract: &str, args: Vec<String>) -> eyre::Result<()> {
        let mut cfg = restore_cfg()?;
        cfg.add_contract(contract.into(), args)?;
        Ok(())
    }

    pub fn remove_contract(contract: &str) -> eyre::Result<()> {
        let mut cfg = restore_cfg()?;
        cfg.remove_contract(contract.into())?;
        Ok(())
    }

    pub fn list() -> eyre::Result<()> {
        let cfg = restore_cfg()?;
        cfg.list();
        Ok(())
    }

    pub fn clean() -> eyre::Result<()> {
        let mut cfg = restore_cfg()?;
        cfg.clean()?;
        Ok(())
    }

    pub async fn run(mut self) -> eyre::Result<()> {
        let cfg = restore_cfg()?;
        self.set_config(cfg);
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

#[cfg(test)]
mod tests {
    use super::*;

    mod util {
        use std::fs;

        use crate::utils::is_existed;

        pub fn create_sol_files() -> std::io::Result<()> {
            for i in 1..100 {
                let dst = String::new() + "examples/contract" + &i.to_string() + ".sol";
                if is_existed(&dst) {
                    println!("{} already exists", dst);
                    continue;
                }
                fs::copy("examples/contract.sol", dst.as_str())?;
            }
            Ok(())
        }

        pub fn delete_sol_files() -> std::io::Result<()> {
            for i in 1..100 {
                let dst = String::new() + "examples/contract" + &i.to_string() + ".sol";
                if is_existed(&dst) {
                    fs::remove_file(dst.as_str()).unwrap();
                }
            }
            Ok(())
        }

        #[test]
        pub fn test_create_sol_files() {
            create_sol_files().unwrap();
            delete_sol_files().unwrap();
        }
    }

    #[tokio::test]
    async fn test_batch_run_success() {
        // given
        util::create_sol_files().unwrap();

        // changed to local rpc later
        let mut cfg = Config {
            rpc_url: Some(dotenv!("RPC_URL").to_string()),
            pri_key: Some(dotenv!("PRI_KEY").to_string()),
            contracts: vec![],
        };

        for i in 1..100 {
            let contract =
                String::new() + "examples/contract" + &i.to_string() + ".sol:SimpleStorage";
            let args = String::new() + "value" + &i.to_string();
            cfg.add_contract(contract, vec![args]).unwrap();
        }

        // when
        let mut executer = Executer::new();
        executer.set_config(cfg.clone());
        executer.run().await.unwrap();

        // clean
        for i in 1..100 {
            let contract =
                String::new() + "examples/contract" + &i.to_string() + ".sol:SimpleStorage";
            let args = String::new() + "value" + &i.to_string();
            cfg.remove_contract(contract).unwrap();
        }
        util::delete_sol_files().unwrap();
    }
}
