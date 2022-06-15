extern crate clap;

use std::{path::PathBuf, ffi::OsString};

use clap::{Arg, App};
use morge::{args::cli, config::restore, config::Config};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            println!("init config file");
            Config::new();
        }
        Some(("set", sub_matches)) => {
            println!("set rpc-url and privatekey");
            let rpc = sub_matches.value_of("rpc-url").expect("set rpc failed");
            println!("The rpc-url is: {}", rpc);

            let key = sub_matches.value_of("pri-key").expect("set private key failed");
            println!("The pri-key is: {}", key);

            let mut cfg = restore()?;
            cfg.set_rpc_and_key(rpc.to_string().clone(), key.to_string().clone())?;
        }
        Some(("add", sub_matches)) => {
            let sol_file = sub_matches.value_of("file").expect("get sol file failed");
            println!("The sol_file is: {}", sol_file);

            let args = sub_matches
                .get_many::<String>("args")
                // .get_raw("args")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .to_vec();
            println!("Calling out to {:?}", args);

            let cfg = restore()?;
            cfg.add_solidity_path(sol_file.into(), args)?;
        }
        Some(("remove", sub_matches)) => {
            let sol_file = sub_matches.value_of("file").expect("get sol file failed");
            println!("The to removed contract is: {}", sol_file);

            let mut cfg = restore()?;
            cfg.remove_solidity_path(sol_file.into())?;
        }
        Some(("deploy", sub_matches)) => {
            println!(
                "start deploy to {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
            let cfg = restore()?;
            cfg.run()?;
        }
        Some(("verify", sub_matches)) => {
            let addr = sub_matches.value_of("rpc-url").expect("get addr failed");
            println!("The addr is: {}", addr);
        }
        Some(("list", _sub_matches)) => {
            println!("list the added contracts files");
            let cfg = restore()?;
            cfg.list();
        }
        Some(("clean", _sub_matches)) => {
            println!("clean deployed contracts cache");
            let mut cfg = restore()?;
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
