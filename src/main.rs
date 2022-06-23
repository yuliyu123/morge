extern crate clap;

use morge::{args::cli, config::restore_cfg, config::Config, Executer};
use std::{env, ffi::OsString};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            println!("init config file");
            Config::new()?;
        }
        Some(("set", sub_matches)) => {
            let rpc = sub_matches.value_of("rpc-url").expect("set rpc failed");
            let key = sub_matches
                .value_of("private-key")
                .expect("set private key failed");

            let mut cfg = restore_cfg()?;
            cfg.set_rpc_and_key(rpc.to_string(), key.to_string())?;
        }
        Some(("add", sub_matches)) => {
            let contract = env::current_dir()?.to_str().unwrap().to_string()
                + "/"
                + sub_matches
                    .value_of("contract")
                    .expect("get sol file failed");
            println!("add contract: {}", contract);

            let args = sub_matches
                .get_many::<String>("args")
                .into_iter()
                .flatten()
                .map(|item| format!("{item:?}"))
                .collect::<Vec<String>>();
            println!("Adding {:?}", args);

            let mut cfg = restore_cfg()?;
            cfg.add_contract(contract.into(), args)?;
        }
        Some(("remove", sub_matches)) => {
            let contract = env::current_dir()?.to_str().unwrap().to_string()
                + "/"
                + sub_matches
                    .value_of("contract")
                    .expect("get sol file failed");

            let mut cfg = restore_cfg()?;
            cfg.remove_contract(contract.into())?;
        }
        Some(("deploy", _sub_matches)) => {
            println!("start deploy");
            let cfg = restore_cfg()?;
            let mut executor = Executer::new();
            executor.set_config(cfg);
            executor.run().await?;
        }
        Some(("verify", sub_matches)) => {
            let addr = sub_matches.value_of("rpc-url").expect("get addr failed");
            println!("The addr is: {}", addr);
        }
        Some(("list", _sub_matches)) => {
            println!("list the added contracts files");
            let cfg = restore_cfg()?;
            cfg.list();
        }
        Some(("clean", _sub_matches)) => {
            println!("clean deployed contracts cache");
            let mut cfg = restore_cfg()?;
            cfg.clean()?;
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
    Ok(())
}
