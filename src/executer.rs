use crate::config::{restore_cfg, save, Config};
use crate::utils::parse::*;
use crate::verify::Verify;
use ethers::utils::Anvil;

pub struct Executer {
    pub cfg: Config,
}

impl Executer {
    pub fn new() -> Self {
        Self {
            cfg: Config {
                rpc_url: None,
                pri_key: None,
                contracts: vec![],
            },
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

    pub fn list() {
        Config::list();
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
                let anvil = &Anvil::new().spawn();
                let provider = get_provider(
                    anvil,
                    self.cfg.rpc_url.unwrap_or_else(|| "".to_string()),
                    self.cfg.pri_key.unwrap_or_else(|| "".to_string()),
                )
                .await;
                // let provider = Arc::new(provider);

                for mut contract in self.cfg.contracts {
                    contract.run(provider.clone()).await?;
                }
                return Ok(());
            }
            false => return Err(eyre::eyre!("no contract to run")),
        }
    }

    pub async fn verify_tx(chain: &str, tx: &str) -> eyre::Result<()> {
        Verify::verify_tx(chain, tx).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod util {
        use std::{fs, u8};

        use crate::utils::fs::*;

        pub fn create_sol_files(num: u8) -> std::io::Result<()> {
            for i in 0..num {
                let dst = String::new() + "examples/contract" + &i.to_string() + ".sol";
                if is_existed(&dst) {
                    println!("{} already exists", dst);
                    continue;
                }
                fs::copy("examples/contract.sol", dst.as_str())?;
            }
            Ok(())
        }

        pub fn delete_sol_files(num: u8) -> std::io::Result<()> {
            for i in 0..num {
                let dst = String::new() + "examples/contract" + &i.to_string() + ".sol";
                if is_existed(&dst) {
                    fs::remove_file(dst.as_str()).unwrap();
                }
            }
            Ok(())
        }

        #[test]
        #[ignore]
        pub fn test_create_sol_files() {
            create_sol_files(100).unwrap();
            delete_sol_files(100).unwrap();
        }
    }

    #[tokio::test]
    async fn test_batch_contracts_run_success() {
        // util::delete_sol_files().unwrap();
        // Executer::clean().unwrap();
        // given
        util::create_sol_files(100).unwrap();

        // changed to local anvil rpc
        let mut cfg = Config {
            rpc_url: Some("".to_string()),
            pri_key: Some("".to_string()),
            contracts: vec![],
        };

        let num: u8 = 100;
        for i in 0..num {
            let contract =
                String::new() + "examples/contract" + &i.to_string() + ".sol:SimpleStorage";
            let args = String::new() + "value" + &i.to_string();
            cfg.add_contract(contract, vec![args]).unwrap();
        }

        // when
        let mut executer = Executer::new();
        executer.set_config(cfg.clone());
        executer.run().await.unwrap();

        // then clean
        util::delete_sol_files(num).unwrap();
        Executer::clean().unwrap();
    }
}
