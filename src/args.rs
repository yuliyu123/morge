use std::ffi::OsString;
use std::path::PathBuf;

use clap::{arg, Command, Arg, App};

pub fn cli() -> Command<'static> {
    Command::new("morge")
        .about("A batch of contracts deployment CLI, support evm compatiable chains")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("init")
                .about("init deploy config")
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("adds contract files and set constructor args")
                .arg(Arg::with_name("file")
                    .short('f')
                    .long("file")
                    .takes_value(true)
                    .help("A cool file"))
                .arg(Arg::with_name("args")
                    .long("args")
                    .takes_value(true)
                    .help("Five less than your favorite number"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove")
                .about("remove contract files")
                .arg(Arg::with_name("contract-name")
                    .long("contract-name")
                    .takes_value(true)
                    .help("set to removed contract name"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("set")
                .about("set rpc url and private key")
                .arg(Arg::with_name("rpc-url")
                    .short('u')
                    .long("rpc-url")
                    .takes_value(true)
                    .help("set rpc url"))
                .arg(Arg::with_name("private-key")
                    .short('k')
                    .long("private-key")
                    .takes_value(true)
                    .help("set private key"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("deploy")
                .about("the chain to deploy")
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("verify")
                .about("verify contract state")
                .arg(Arg::with_name("addr")
                    .long("addr")
                    .takes_value(true)
                    .help("verify contract state"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("clean")
                .about("clean the deploy cache")
        )
        .subcommand(
            Command::new("list")
                .about("list the added contract files")
        )
}


#[test]
fn test_morge_args() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        Some(("add", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap();
            match stash_command {
                ("-f", sub_matches) => {
                    let sol_file = sub_matches.get_one::<String>("sol_file");
                    println!("add sol_file {:?}", sol_file);
                }
                ("-args", sub_matches) => {
                    let args = sub_matches
                    .get_many::<PathBuf>("PATH")
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                    println!("constructor args {:?}", args);
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        Some(("deploy", sub_matches)) => {
            println!(
                "deploy to {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        Some(("verify", sub_matches)) => {
            println!(
                "verify addr {}",
                sub_matches.get_one::<String>("ADDR").expect("required")
            );
        }
        Some(("list", sub_matches)) => {
            println!("list the added contracts files");
        }
        Some(("clean", sub_matches)) => {
            println!("clean deployed contracts cache");
        }
        Some(("set", sub_matches)) => {
            println!("set rpc-url and privatekey");
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
}
