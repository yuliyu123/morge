use core::panic;
use ethers::prelude::*;
use std::collections::HashMap;

pub struct Verify;

impl Verify {
    fn get_chainnet(chain: &str) -> Chain {
        let mut chains_map = HashMap::new();
        chains_map.insert("eth", Chain::Mainnet);
        chains_map.insert("ropsten", Chain::Ropsten);
        chains_map.insert("rinkeby", Chain::Rinkeby);
        chains_map.insert("kovan", Chain::Kovan);
        chains_map.insert("goerli", Chain::Goerli);
        match chains_map.contains_key(chain) {
            true => chains_map.get(chain).unwrap().clone(),
            false => Chain::Mainnet,
        }
    }

    fn get_api_key(chain: &str) -> &str {
        let key = "YRFQ5PZHZ888THDP27H4B671QYW5X4BBTU";
        let mut keys_map = HashMap::new();
        keys_map.insert("eth", key);
        keys_map.insert("rinkeby", key);
        keys_map.insert("kovan", key);
        keys_map.insert("ropsten", key);
        keys_map.insert("goerli", key);
        match keys_map.contains_key(chain) {
            true => keys_map.get(chain).unwrap().clone(),
            false => panic!("{} chain api key not found", chain),
        }
    }

    pub async fn verify_tx(chain: &str, tx: &str) -> eyre::Result<bool> {
        let chainnet = Verify::get_chainnet(chain);
        let key = Verify::get_api_key(chain);
        let client = Client::new(chainnet, key).unwrap();

        let status = client.check_contract_execution_status(tx).await;
        match status {
            Ok(_) => Ok(true),
            Err(err) => panic!("Verify tx: {} failed, err: {:?}", tx, err),
        }
    }
}

#[tokio::test]
#[ignore = "maybe failed due to China gov firewall"]
async fn test_verify() -> eyre::Result<()> {
    // verify(Chain::Mainnet, "I5BXNZYP5GEDWFINGVEZKYIVU2695NPQZB".to_string(), "0x20838f43529f3fe0658eef6d2ded1184df317ed816c61beb70d38a6a02372852".into()).await?;
    let res = Verify::verify_tx(
        "rinkeby",
        "0xc6e08d3b5b1077f4662907fa547fab34bac033a0501655aca0b903057c118da8",
    )
    .await?;
    assert!(res);
    Ok(())
}
