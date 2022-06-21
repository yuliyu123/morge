use std::fs;
use std::fs::{OpenOptions};
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use ethers::core::{
    abi::{
        token::{LenientTokenizer, StrictTokenizer, Tokenizer}, ParamType, Token,
    },
    types::*,
};
use ethers::prelude::{Provider, RetryClient, Http, LocalWallet};
use eyre::{Result, WrapErr, eyre};
use std::str::FromStr;

pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

pub fn create_dir(dir: &Path) -> io::Result<()> {
    match fs::create_dir(dir) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}

/// Parses string input as Token against the expected ParamType
// #[allow(clippy::no_effect)]
pub fn parse_tokens<'a, I: IntoIterator<Item = (&'a ParamType, &'a str)>>(
    params: I,
    lenient: bool,
) -> Result<Vec<Token>> {
    params
        .into_iter()
        .map(|(param, value)| {
            let mut token = if lenient {
                LenientTokenizer::tokenize(param, value)
            } else {
                StrictTokenizer::tokenize(param, value)
            };

            if token.is_err() && value.starts_with("0x") {
                match param {
                    ParamType::FixedBytes(32) => {
                        if value.len() < 66 {
                            let padded_value = [value, &"0".repeat(66 - value.len())].concat();
                            token = if lenient {
                                LenientTokenizer::tokenize(param, &padded_value)
                            } else {
                                StrictTokenizer::tokenize(param, &padded_value)
                            };
                        }
                    }
                    ParamType::Uint(_) => {
                        // try again if value is hex
                        if let Ok(value) = U256::from_str(value).map(|v| v.to_string()) {
                            token = if lenient {
                                LenientTokenizer::tokenize(param, &value)
                            } else {
                                StrictTokenizer::tokenize(param, &value)
                            };
                        }
                    }
                    // TODO: Not sure what to do here. Put the no effect in for now, but that is not
                    // ideal. We could attempt massage for every value type?
                    _ => {}
                }
            }
            token
        })
        .collect::<Result<_, _>>()
        .wrap_err("Failed to parse tokens")
}

pub fn is_existed(path: &String) -> bool{
    let path = Path::new(path);
    return path.exists();
}


/// Gives out a provider with a `100ms` interval poll if it's a localhost URL (most likely an anvil
/// node) and with the default, `7s` if otherwise.
pub fn get_http_provider(url: &str, aggressive: bool) -> Arc<Provider<RetryClient<Http>>> {
    let (max_retry, initial_backoff) = if aggressive { (1000, 1) } else { (10, 1000) };

    let provider = Provider::<RetryClient<Http>>::new_client(url, max_retry, initial_backoff)
        .expect("Bad fork provider.");

    Arc::new(if url.contains("127.0.0.1") || url.contains("localhost") {
        provider.interval(Duration::from_millis(100))
    } else {
        provider
    })
}

pub fn get_from_private_key(private_key: &str) -> Result<LocalWallet> {
    let privk = private_key.strip_prefix("0x").unwrap_or(private_key);
    LocalWallet::from_str(privk)
        .map_err(|x| eyre!("Failed to create wallet from private key: {x}"))
}

pub fn is_contract_existed(contract: String) -> bool {
    println!("contract: {}", contract);
    tracing::info!("add contract:  {}", contract);

    let contract_vec = contract.split(":").collect::<Vec<&str>>();
    if !is_existed(&contract_vec[0].into()) {
        return false;
    }
    true
}
