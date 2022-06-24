use ethers::prelude::k256::ecdsa::SigningKey;
use ethers::utils::AnvilInstance;
use ethers::{
    abi::Constructor,
    core::{
        abi::{
            token::{LenientTokenizer, StrictTokenizer, Tokenizer},
            ParamType, Token,
        },
        types::*,
    },
    prelude::*,
};
use eyre::{eyre, Result, WrapErr};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

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

pub fn parse_constructor_args(
    constructor: &Constructor,
    constructor_args: &[String],
) -> Result<Vec<Token>> {
    let params = constructor
        .inputs
        .iter()
        .zip(constructor_args)
        .map(|(input, arg)| (&input.kind, arg.as_str()))
        .collect::<Vec<_>>();

    parse_tokens(params, true)
}

/// Gives out a provider with a `100ms` interval poll if it's a localhost URL (most likely an anvil
/// node) and with the default, `7s` if otherwise.
#[allow(dead_code)]
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
    LocalWallet::from_str(privk).map_err(|x| eyre!("Failed to create wallet from private key: {x}"))
}

// ) -> Arc<SignerMiddleware<Arc<Provider<RetryClient<Http>>>, Wallet<SigningKey>>> {
pub async fn get_provider(
    anvil: &AnvilInstance,
    rpc_url: String,
    pri_key: String,
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    if rpc_url.is_empty() || pri_key.is_empty() || rpc_url.contains("http://localhost:8545") {
        let provider = Provider::<Http>::try_from(anvil.endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64));
        let wallet: LocalWallet = anvil.keys()[0].clone().into();
        let provider = SignerMiddleware::new(provider.clone(), wallet);
        return provider;
    }

    let provider = Provider::<Http>::try_from(rpc_url)
        .unwrap()
        .interval(Duration::from_millis(10u64));
    let wallet = get_from_private_key(&pri_key.as_str());
    let chain_id = provider.get_chainid().await.unwrap();
    let wallet = wallet.unwrap().with_chain_id(chain_id.as_u64());
    let provider = SignerMiddleware::new(provider.clone(), wallet);
    // Arc::new(provider)
    provider
}

#[allow(dead_code)]
pub fn get_anvil_provider(anvil: &AnvilInstance, idx: usize) -> Provider<Http> {
    let sender = anvil.addresses()[idx];
    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64))
        .with_sender(sender);
    provider
}
