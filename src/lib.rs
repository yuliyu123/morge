pub mod args;
pub mod config;
pub mod deploy;
pub mod utils;
// mod multi_task;

static INIT_PATH: &str = ".morge";
static INIT_CFG: &str = ".morge/config.json";

pub enum Err {
    CompileError(String),
    DeployError(String),
}
