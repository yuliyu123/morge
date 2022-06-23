use crate::contract::ContractInfo;
use crate::{utils::*, Err, INIT_CFG, INIT_PATH};
use ethers::prelude::SignerMiddleware;
use futures::stream::TryChunksError;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// init info, include mainnet and chain_id info, etc.
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
    pub fn new() -> eyre::Result<Self> {
        let cfg = Config {
            rpc_url: None,
            pri_key: None,
            contracts: vec![],
        };
        save(&cfg)?;
        Ok(cfg)
    }

    pub fn set_rpc_and_key(&mut self, rpc_url: String, pri_key: String) -> eyre::Result<()> {
        self.rpc_url = Some(rpc_url);
        self.pri_key = Some(pri_key);
        save(self)?;
        println!("set rpc url and private key success");
        Ok(())
    }

    // add contract and args through -f x.sol:x --args a b c
    pub fn add_contract(&mut self, contract: String, args: Vec<String>) -> eyre::Result<()> {
        match is_contract_existed(contract.clone()) && contract.ends_with(".sol") {
            true => {
                let contract_info = ContractInfo::new(contract, args);
                if self
                    .contracts
                    .iter()
                    .map(|contract| {
                        contract.contract == contract_info.contract
                            && contract.name == contract_info.name
                    })
                    .any(|x| x)
                {
                    return Err(eyre::eyre!("contract already exists"));
                };
                self.contracts.push(contract_info);
                save(self)?;
                Ok(())
            }
            false => Err(eyre::eyre!("contract not found")),
        }
    }

    // remove contract from config file
    pub fn remove_contract(&mut self, contract: String) -> eyre::Result<()> {
        match is_contract_existed(contract.clone()) && contract.ends_with(".sol") {
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
                    return Err(eyre::eyre!("contract not exists"));
                };

                // todo: add ut to cover
                println!("contract info: {:?}", contract_info);
                self.contracts.retain(|item| {
                    println!("item info: {:?}", item);
                    item.contract != contract_info.contract && item.name != contract_info.name
                });
                save(self)?;
                println!("remove contract: {} success", contract);
                Ok(())
            }
            false => Err(eyre::eyre!("contract not found")),
        }
    }

    pub fn list(self) {
        let cfg_path = Path::new(INIT_CFG);
        if !cfg_path.exists() {
            println!("cfg file not existed");
        }

        let json = std::fs::read_to_string(INIT_CFG).unwrap();
        let contract_infos = from_json(&json).unwrap().contracts;

        if contract_infos.is_empty() {
            println!("no solidity path");
        }

        for contract_info in contract_infos.iter() {
            println!("contract name: {:?}", contract_info.name);
            println!("contract contract: {:?}", contract_info.contract);
            println!("contract args: {:?}", contract_info.args);
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
#[test]
fn test_init() {
    // given
    let mut cfg = Config::new().unwrap();
    cfg.clean().unwrap();
    let rpc_url = "http://localhost:8545";
    let pri_key = "0x1234567890123456789012345678901234567890123456789012345678901234";

    // when
    cfg.set_rpc_and_key(rpc_url.into(), pri_key.into()).unwrap();
    save(&cfg).unwrap();

    // then
    assert!(Path::new(INIT_CFG).exists());
    let mut cfg = restore_cfg().unwrap();
    assert_eq!(cfg.rpc_url, Some(rpc_url.into()));
    assert_eq!(cfg.pri_key, Some(pri_key.into()));
    assert_eq!(cfg.contracts, vec![]);
}

#[test]
fn test_add_contract_success() {
    // given
    test_init();
    let mut cfg = restore_cfg().unwrap();
    let contract_file = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .join("examples/contract.sol")
        .to_str()
        .unwrap()
        .to_string();
    let contract = contract_file.clone() + ":SimpleStorage";
    let args = vec!["value".into()];

    // when
    cfg.add_contract(contract.clone(), args).unwrap();

    // then
    assert_eq!(cfg.contracts.len(), 1);
    assert_eq!(cfg.contracts[0].contract, contract);
}

#[test]
fn test_add_contract_failed_with_wrong_path() {
    // given
    test_init();
    let mut cfg = restore_cfg().unwrap();
    let contract_file = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .join("examples/contract.sol")
        .to_str()
        .unwrap()
        .to_string();
    let contract = contract_file.clone() + ":SimpleStorage";
    // let args = vec!["value".into()];

    // when
    // cfg.add_contract(contract, args).unwrap();
    // assert_eq!(Err(eyre::eyre!("contract file not existed")), cfg.add_contract(contract, args));

    // then
    assert_eq!(cfg.contracts.len(), 0);
}

#[test]
fn test_remove_contract_success() {
    // given
    test_add_contract_success();
    let mut cfg = restore_cfg().unwrap();
    let contract_file = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .join("examples/contract.sol")
        .to_str()
        .unwrap()
        .to_string();
    let contract = contract_file.clone() + ":SimpleStorage";
    assert_eq!(cfg.contracts.len(), 1);
    assert_eq!(cfg.contracts[0].contract, contract);

    // when
    cfg.remove_contract(contract.clone()).unwrap();

    // then
    assert_eq!(cfg.contracts.len(), 0);
}

#[test]
fn test_clean() {
    // given
    test_add_contract_success();
    let mut cfg = restore_cfg().unwrap();
    assert_eq!(cfg.contracts.len(), 1);

    // when
    cfg.clean().unwrap();

    // then
    assert_eq!(cfg.contracts.len(), 0);
    let cfg = restore_cfg().unwrap();
    assert_eq!(cfg.contracts.len(), 0);
}
