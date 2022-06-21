extern crate clap;

use std::ffi::OsString;
use morge::{args::cli, config::restore_cfg, config::Config, Executer};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            println!("init config file");
            Config::new()?;
        }
        Some(("set", sub_matches)) => {
            println!("set rpc-url and privatekey");
            let rpc = sub_matches.value_of("rpc-url").expect("set rpc failed");
            println!("The rpc-url is: {}", rpc);

            let key = sub_matches.value_of("private-key").expect("set private key failed");
            println!("The pri-key is: {}", key);

            let mut cfg = restore_cfg()?;
            cfg.set_rpc_and_key(rpc.to_string().clone(), key.to_string().clone())?;
        }
        Some(("add", sub_matches)) => {
            let contract = sub_matches.value_of("contract").expect("get sol file failed");
            println!("The contract is: {}", contract);

            let args = sub_matches.get_many::<String>("args")
            .into_iter()
            .flatten()
            .map(|item| format!("{item:?}"))
            .collect::<Vec<String>>();
            println!("Adding {:?}", args);

            let mut cfg = restore_cfg()?;
            cfg.add_contract(contract.into(), args)?;
        }
        Some(("remove", sub_matches)) => {
            let sol_file = sub_matches.value_of("contract").expect("get contract failed");
            println!("The to removed contract is: {}", sol_file);

            let mut cfg = restore_cfg()?;
            cfg.remove_contract(sol_file.into())?;
        }
        Some(("deploy", sub_matches)) => {
            println!(
                "start deploy to {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
            let cfg = restore_cfg()?;
            let mut executor = Executer::new();
            executor.cfg = cfg;
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
