use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::File;
use std::fs;
use crate::deploy::ContractInfo;
use crate::{INIT_PATH, INIT_CFG, utils::*};
use futures::future::ok;
use serde::{Serialize, Deserialize};
use serde_json::Result;

// init info, include mainnet and chain_id info, etc.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub pri_key: String,
    pub contracts: Vec<ContractInfo>,
}

fn from_json(json: &str) -> Result<Config> {
    serde_json::from_str(json)
}

fn to_json(cfg: &Config) -> Result<String> {
    serde_json::to_string_pretty(cfg)
}

pub fn save(cfg: &Config) -> eyre::Result<()> {
    let json = to_json(cfg);
    let mut file = File::create(Path::new(INIT_CFG))?;
    file.write_all(json?.as_bytes())?;
    Ok(())
}

pub fn restore() -> eyre::Result<Config> {
    let file_str = fs::read_to_string(INIT_CFG)?;
    Ok(from_json(&file_str)?)
}

impl Config {
    pub fn new() -> eyre::Result<()> {
        let cfg = Config {
            rpc_url: "http://localhost:8545".into(),
            pri_key: "".into(),
            contracts: vec![],
        };
        to_json(&cfg)?;
        Ok(())
    }

    pub fn set_rpc_and_key(&mut self, rpc_url: String, pri_key: String) -> eyre::Result<()> {
        self.rpc_url = rpc_url;
        self.pri_key = pri_key;
        Ok(())
    }

    pub fn add_solidity_path(&mut self, path: String, args: Vec<String>) -> eyre::Result<()> {
        // for path in paths {
        //     println!("path: {}", path);
        //     tracing::info!("starting tests");
        //     if !path.is_empty() && is_existed(&path)?.0 && !self.contracts.contains(&path) {
        //         // self.contracts.push(path.clone());
        //     }
        // }

        // let json = self.to_json()?;
        // std::fs::write(INIT_CFG, json)?;
        Ok(())
    }

    // morge remove files
    pub fn remove_solidity_path(&mut self, removed_path: String) -> eyre::Result<()> {
        if self.contracts.is_empty() {
            return Ok(());
        }
        // let mut contracts_path = self.contracts.to_vec();
        // contracts_path.retain(|path| !removed_paths.contains(path));
        // let json = self.to_json()?;
        // std::fs::write(INIT_CFG, json)?;
        Ok(())
    }

    pub fn list(&self) {
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
            println!("contract path: {:?}", contract_info.path);
            println!("contract args: {:?}", contract_info.args);
        };
    }

    pub fn clean(&mut self) -> eyre::Result<()> {
        self.rpc_url = "".into();
        self.pri_key = "".into();
        Ok(())
    }

    pub fn run(self) -> eyre::Result<()> {
        // self.contracts.iter().map(|&mut contract_info| contract_info.run(&mut contract_info));
        Ok(())
    }
}

// #[test]
// fn test_init_ser_deseralize() {
//     let mut cfg = Config::new("https://rinkeby.infura.io/v3/c8c81708601f4c6ca0ad9b0c7bb1911f".into(), "xxx".into());
//     let json_str = cfg.to_json();

//     let cfg2 = Config::from_json(&json_str).unwrap();
//     let json_str2 = cfg2.to_json().unwrap();
//     assert_eq!(json_str, json_str2);
// }

// #[test]
// fn test_init_() {
//     let mut cfg = Config::new("https://rinkeby.infura.io/v3/c8c81708601f4c6ca0ad9b0c7bb1911f".into(), "xxx".into());
//     cfg.save().unwrap();
//     assert!(cfg.contracts_path.is_empty());

//     let mut paths = vec![Path::new(&env!("CARGO_MANIFEST_DIR")).join("src/examples/contract.sol").to_str().unwrap().to_string()];
//     cfg.add_solidity_path(&mut paths);
//     assert_eq!(cfg.contracts_path.len(), 1);

//     // cfg.remove_solidity_path(&mut paths);
//     // assert_eq!(cfg.contracts_path.len(), 0);
// }
