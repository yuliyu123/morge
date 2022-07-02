use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::contract::ContractInfo;
use crate::utils::fs::*;
use crate::{INIT_CFG, INIT_PATH};

// init config file, include rpc url、private key and contracts.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub rpc_url: Option<String>,
    pub pri_key: Option<String>,
    pub contracts: Vec<ContractInfo>,
}

fn from_json(json: &str) -> Result<Config> {
    serde_json::from_str(json)
}

fn to_json(cfg: &Config) -> Result<String> {
    serde_json::to_string_pretty(cfg)
}

pub fn save(cfg: &Config) -> eyre::Result<()> {
    if !is_existed(&INIT_PATH.into()) {
        fs::create_dir(INIT_PATH)?;
    }
    let json = to_json(cfg);
    let mut file = File::create(Path::new(INIT_CFG))?;
    file.write_all(json?.as_bytes())?;
    Ok(())
}

pub fn restore_cfg() -> eyre::Result<Config> {
    let file_str = fs::read_to_string(INIT_CFG)?;
    let cfg = from_json(&file_str)?;
    Ok(cfg)
}

impl Config {
    pub fn new() -> Self {
        Config {
            rpc_url: None,
            pri_key: None,
            contracts: vec![],
        }
    }

    pub fn set_rpc_and_key(&mut self, rpc_url: String, pri_key: String) -> eyre::Result<()> {
        self.rpc_url = Some(rpc_url);
        self.pri_key = Some(pri_key);
        save(self)?;
        println!("Set rpc url and private key success");
        Ok(())
    }

    // add contract and args by specify -f x.sol:x --args a b c
    pub fn add_contract(&mut self, contract: String, args: Vec<String>) -> eyre::Result<()> {
        match is_contract_existed(contract.clone()) {
            true => {
                let contract_info = ContractInfo::new(contract.clone(), args.clone());
                if self
                    .contracts
                    .iter()
                    .map(|contract| {
                        contract.contract == contract_info.contract
                            && contract.name == contract_info.name
                    })
                    .any(|x| x)
                {
                    println!("Contract {} already existed", contract);
                    return Ok(());
                };
                self.contracts.push(contract_info);
                save(self)?;
                println!("Add contract {} and args: {:?} success", contract, args);
                Ok(())
            }
            false => {
                println!("Contract {} not found", contract);
                Ok(())
            }
        }
    }

    // remove contract from config file
    pub fn remove_contract(&mut self, contract: String) -> eyre::Result<()> {
        match is_contract_existed(contract.clone()) {
            true => {
                let contract_info = ContractInfo::new(contract.clone(), vec![]);
                if !self
                    .contracts
                    .iter()
                    .map(|contract| {
                        contract.contract == contract_info.contract
                            && contract.name == contract_info.name
                    })
                    .any(|x| x)
                {
                    println!("Contract {} not exists", contract);
                    return Ok(());
                };

                self.contracts.retain(|item| {
                    !(item.contract == contract_info.contract && item.name == contract_info.name)
                });
                save(self)?;
                println!("Remove contract: {} success", contract);
                Ok(())
            }
            false => {
                println!("Contract {} not found", contract);
                Ok(())
            }
        }
    }

    pub fn list() {
        let cfg_path = Path::new(INIT_CFG);
        if !cfg_path.exists() {
            println!("Configuration file not existed");
            return;
        }

        let cfg = restore_cfg().unwrap();
        if cfg.rpc_url.is_none() {
            println!("Rpc url not set, please set");
            return;
        }

        if cfg.pri_key.is_none() {
            println!("Private key not set, please set");
            return;
        }

        let contracts = cfg.contracts;
        if contracts.is_empty() {
            println!("Have no any contract file to deploy.");
            return;
        }

        for contract_info in contracts.iter() {
            println!("Contract name: {:?}", contract_info.name);
            println!("Contract contract: {:?}", contract_info.contract);
            println!("Contract args: {:?} \n", contract_info.args);
        }
    }

    pub fn clean(&mut self) -> eyre::Result<()> {
        self.rpc_url = None;
        self.pri_key = None;
        self.contracts = vec![];
        save(self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn setup() {
        // given
        let mut cfg = Config::new();
        let rpc_url = "http://localhost:8545";
        let pri_key = "0x1234567890123456789012345678901234567890123456789012345678901234";

        // when
        cfg.set_rpc_and_key(rpc_url.into(), pri_key.into()).unwrap();
        save(&cfg).unwrap();

        // then
        assert!(Path::new(INIT_CFG).exists());
        let cfg = restore_cfg().unwrap();
        assert_eq!(cfg.rpc_url, Some(rpc_url.into()));
        assert_eq!(cfg.pri_key, Some(pri_key.into()));
        assert_eq!(cfg.contracts, vec![]);
    }

    fn teardown() {
        let mut cfg = restore_cfg().unwrap();
        cfg.clean().unwrap();
        let cfg = restore_cfg().unwrap();
        assert_eq!(cfg.contracts.len(), 0);
    }

    #[test]
    fn test_init() {
        setup();
        teardown();
    }

    #[test]
    fn test_add_contract_success() {
        // given
        setup();
        let mut cfg = restore_cfg().unwrap();
        let contract_file = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("examples/contract.sol:SimpleStorage")
            .to_str()
            .unwrap()
            .to_string();
        let args = vec!["value".into()];

        // when
        cfg.add_contract(contract_file.clone(), args).unwrap();

        // then
        assert_eq!(cfg.contracts.len(), 1);
        assert_eq!(
            cfg.contracts[0].contract,
            Path::new(&env!("CARGO_MANIFEST_DIR"))
                .join("examples/contract.sol")
                .to_str()
                .unwrap()
                .to_string()
        );
        teardown();
    }

    #[test]
    #[should_panic]
    fn test_add_contract_failed_with_wrong_path() {
        // given
        test_init();
        let mut cfg = restore_cfg().unwrap();
        let contract_file = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("examples/contract.sol")
            .to_str()
            .unwrap()
            .to_string();
        let args = vec!["value".into()];

        // when
        // should panic
        cfg.add_contract(contract_file.clone(), args).unwrap();
    }

    #[test]
    fn test_remove_contract_success() {
        // given
        setup();
        let mut cfg = restore_cfg().unwrap();

        let contract_01 = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("examples/contract.sol:SimpleStorage_01")
            .to_str()
            .unwrap()
            .to_string();
        let args = vec!["value".into()];
        cfg.add_contract(contract_01.clone(), args).unwrap();

        let contract_02 = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("examples/contract.sol:SimpleStorage_02")
            .to_str()
            .unwrap()
            .to_string();
        let args = vec!["value".into()];
        cfg.add_contract(contract_02.clone(), args).unwrap();
        assert_eq!(cfg.contracts.len(), 2);

        // when
        let contract_file = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("examples/contract.sol:SimpleStorage_01")
            .to_str()
            .unwrap()
            .to_string();
        cfg.remove_contract(contract_file.clone()).unwrap();

        // then
        assert_eq!(cfg.contracts.len(), 1);
        assert_eq!(cfg.contracts[0].name, "SimpleStorage_02");
        teardown();
    }

    #[test]
    #[should_panic]
    fn test_remove_contract_failed_with_wrong_path() {
        // given
        setup();
        let mut cfg = restore_cfg().unwrap();
        let contract_file = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("examples/contract.sol:SimpleStorage")
            .to_str()
            .unwrap()
            .to_string();
        let args = vec!["value".into()];
        cfg.add_contract(contract_file.clone(), args).unwrap();
        assert_eq!(cfg.contracts.len(), 1);
        assert_eq!(
            cfg.contracts[0].contract,
            Path::new(&env!("CARGO_MANIFEST_DIR"))
                .join("examples/contract.sol")
                .to_str()
                .unwrap()
                .to_string()
        );

        // when
        let contract_file = Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("examples/contract.sol")
            .to_str()
            .unwrap()
            .to_string();
        cfg.remove_contract(contract_file.clone()).unwrap();

        // then
        assert_eq!(cfg.contracts.len(), 0);
    }
}
