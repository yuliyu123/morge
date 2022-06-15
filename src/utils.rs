use std::fs;
use std::fs::{OpenOptions};
use std::io;
use std::path::Path;
use ethers::core::{
    abi::{
        token::{LenientTokenizer, StrictTokenizer, Tokenizer}, ParamType, Token,
    },
    types::*,
};
use eyre::{Result, WrapErr};
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

pub fn is_existed(path: &String) -> eyre::Result<(bool, String)> {
    let metadata = fs::metadata(path)?;
    match metadata.accessed() {
        Ok(_) => return Ok((true, path.clone())),
        Err(_) => eyre::bail!("{} not existed", path),
    }
    // Ok(())
}
