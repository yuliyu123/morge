use clap::{Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new("morge")
        .about("A batch of contracts deployment CLI, currently support eth、goerli、kovan、rinkeby、ropsten、
        polygon、polygon-mumbai、fantom、fantom-testnet、bsc、bsc-testnet、arbitrum、arbitrum-testnet、
        optimism、optimism-kovan、avalanche、avalanche-fuji chains.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(Command::new("init").about("init deploy config"))
        .subcommand(
            Command::new("add")
                .about("adds contract files and set contract constructor args")
                .arg(
                    Arg::with_name("contract")
                        .short('c')
                        .long("contract")
                        .takes_value(true)
                        .help("specify the contract file"),
                )
                .arg(
                    Arg::with_name("args")
                        .long("args")
                        .takes_value(true)
                        .help("Five less than your favorite number")
                        .multiple_values(true),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("remove")
                .about("remove contract files")
                .arg(
                    Arg::with_name("contract")
                        .short('c')
                        .long("contract")
                        .takes_value(true)
                        .help("set to removed contract name"),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("set")
                .about("set rpc url and private key")
                .arg(
                    Arg::with_name("rpc-url")
                        .short('u')
                        .long("rpc-url")
                        .takes_value(true)
                        .help("set rpc url"),
                )
                .arg(
                    Arg::with_name("private-key")
                        .short('k')
                        .long("private-key")
                        .takes_value(true)
                        .help("set private key"),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("verify")
                .about("verify contract state")
                .arg(
                    Arg::with_name("chain")
                        .short('c')
                        .long("chain")
                        .takes_value(true)
                        .help("provide chainnet name"),
                )
                .arg(
                    Arg::with_name("tx")
                        .short('t')
                        .long("tx")
                        .takes_value(true)
                        .help("provide the transaction hash"),
                )
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("deploy").about("the chain to deploy"))
        .subcommand(Command::new("clean").about("clean the deploy cache"))
        .subcommand(Command::new("list").about("list the added contract files"))
}
