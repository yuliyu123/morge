pub mod args;
pub mod config;
pub mod contract;
pub mod verify;
pub use config::*;

mod executer;
mod utils;
use chrono::Local;
pub use executer::Executer;

#[macro_use]
extern crate dotenv_codegen;
extern crate clap;
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

static INIT_PATH: &str = ".morge";
static INIT_CFG: &str = ".morge/config.json";

pub enum Err {
    CompileError(String),
    DeployError(String),
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
