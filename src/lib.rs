pub mod args;
pub mod config;
pub mod contract;
pub mod verify;
use std::collections::HashMap;

pub use config::*;

mod executer;
mod utils;
use chrono::Local;
use ethers::prelude::Chain;
pub use executer::Executer;

extern crate clap;
extern crate dotenv_codegen;
extern crate log;
extern crate log4rs;

#[macro_use(lazy_static)]
extern crate lazy_static;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

static INIT_PATH: &str = ".morge";
static INIT_CFG: &str = ".morge/config.json";
static MAINNET_KEY: &str = "YRFQ5PZHZ888THDP27H4B671QYW5X4BBTU";
static POLYGON_KEY: &str = "ERXCC9XEMMSSBN7NUAW889MIGEAAXA2MJJ";
static FANTOM_KEY: &str = "YKFZMKCUR78W2QDRJ38V4XXB43Q58GZK1T";
static BSC_KEY: &str = "YYGABQAHU3FICXGNI8IJ823DAAZ4TZZHEH";
static ARBITRUM_KEY: &str = "U1P5XWVA3N77ZNVBDUU6ZYKKNBEGI86WGC";
static OPTIMISM_KEY: &str = "CIC8KPRTUBPQ7VEFCKX9SVDK1HMBWK5G44";
static AVAL_KEY: &str = "3MZTI5W7HSD117B5FGWN8FHWUQGX9GJV4C";

lazy_static! {
    static ref CHAINS_MAP: HashMap<&'static str, Chain> = {
        let mut chains_map = HashMap::new();
        chains_map.insert("eth", Chain::Mainnet);
        chains_map.insert("ropsten", Chain::Ropsten);
        chains_map.insert("rinkeby", Chain::Rinkeby);
        chains_map.insert("kovan", Chain::Kovan);
        chains_map.insert("goerli", Chain::Goerli);
        chains_map.insert("polygon", Chain::Polygon);
        chains_map.insert("polygon-mumbai", Chain::PolygonMumbai);
        chains_map.insert("fantom", Chain::Fantom);
        chains_map.insert("fantom-testnet", Chain::FantomTestnet);
        chains_map.insert("bsc", Chain::BinanceSmartChain);
        chains_map.insert("bsc-testnet", Chain::BinanceSmartChainTestnet);
        chains_map.insert("arbitrum", Chain::Arbitrum);
        chains_map.insert("arbitrum-testnet", Chain::ArbitrumTestnet);
        chains_map.insert("optimism", Chain::Optimism);
        chains_map.insert("optimism-kovan", Chain::OptimismKovan);
        chains_map.insert("avalanche", Chain::Avalanche);
        chains_map.insert("avalanche-fuji", Chain::AvalancheFuji);
        chains_map
    };
    static ref KEYS_MAP: HashMap<&'static str, &'static str> = {
        let mut keys_map = HashMap::new();
        keys_map.insert("eth", MAINNET_KEY);
        keys_map.insert("ropsten", MAINNET_KEY);
        keys_map.insert("rinkeby", MAINNET_KEY);
        keys_map.insert("kovan", MAINNET_KEY);
        keys_map.insert("goerli", MAINNET_KEY);
        keys_map.insert("polygon", POLYGON_KEY);
        keys_map.insert("polygon-mumbai", POLYGON_KEY);
        keys_map.insert("fantom", FANTOM_KEY);
        keys_map.insert("fantom-testnet", FANTOM_KEY);
        keys_map.insert("bsc", BSC_KEY);
        keys_map.insert("bsc-testnet", BSC_KEY);
        keys_map.insert("arbitrum", ARBITRUM_KEY);
        keys_map.insert("arbitrum-testnet", ARBITRUM_KEY);
        keys_map.insert("optimism", OPTIMISM_KEY);
        keys_map.insert("optimism-kovan", OPTIMISM_KEY);
        keys_map.insert("avalanche", AVAL_KEY);
        keys_map.insert("avalanche-fuji", AVAL_KEY);
        keys_map
    };
}

pub fn log_config() -> eyre::Result<()> {
    let log_path = format!(
        "{}/result_{}.log",
        INIT_PATH,
        Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
    );
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(log_path)?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;
    Ok(())
}
