pub mod args;
pub mod config;
pub mod contract;
pub mod utils;
// mod multi_task;
pub use config::Config;

mod executer;
pub use executer::Executer;

#[macro_use]
extern crate dotenv_codegen;

static INIT_PATH: &str = ".morge";
static INIT_CFG: &str = ".morge/config.json";

pub enum Err {
    CompileError(String),
    DeployError(String),
}
