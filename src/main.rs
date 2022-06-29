// use morge::log_config;
use morge::{args::cli, Executer};
use std::{env, ffi::OsString};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // log_config()?;
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", _sub_matches)) => {
            Executer::init()?;
        }
        Some(("set", sub_matches)) => {
            let rpc_url = sub_matches.value_of("rpc-url").expect("set rpc failed");
            let pri_key = sub_matches
                .value_of("private-key")
                .expect("set private key failed");

            Executer::set_rpc_and_key(rpc_url, pri_key)?;
        }
        Some(("add", sub_matches)) => {
            let contract = env::current_dir()?.to_str().unwrap().to_string()
                + "/"
                + sub_matches
                    .value_of("contract")
                    .expect("get sol file failed");
            let args = sub_matches
                .get_many::<String>("args")
                .into_iter()
                .flatten()
                .map(|item| format!("{item:?}"))
                .collect::<Vec<String>>();

            Executer::add_contract(&contract, args)?;
        }
        Some(("remove", sub_matches)) => {
            let contract = env::current_dir()?.to_str().unwrap().to_string()
                + "/"
                + sub_matches
                    .value_of("contract")
                    .expect("get sol file failed");

            Executer::remove_contract(&contract)?;
        }
        Some(("deploy", _sub_matches)) => {
            println!("Start deploy");
            let executor = Executer::new();
            executor.run().await?;
        }
        Some(("verify", sub_matches)) => {
            let chain = sub_matches.value_of("chain").expect("set rpc failed");
            let tx = sub_matches.value_of("tx").expect("get addr failed");
            println!("Chain: {}, tx: {}", chain, tx);
            Executer::verify_tx(chain, tx).await;
        }
        Some(("list", _sub_matches)) => {
            println!("List added contracts files");
            Executer::list();
        }
        Some(("clean", _sub_matches)) => {
            Executer::clean()?;
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
