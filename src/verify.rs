use core::panic;
use ethers::prelude::*;

use crate::{CHAINS_MAP, KEYS_MAP};

pub struct Verify;

impl Verify {
    fn get_chainnet(chain: &str) -> Chain {
        match CHAINS_MAP.contains_key(chain) {
            true => CHAINS_MAP.get(chain).unwrap().clone(),
            false => Chain::Mainnet,
        }
    }

    fn get_api_key(chain: &str) -> &str {
        match KEYS_MAP.contains_key(chain) {
            true => KEYS_MAP.get(chain).unwrap().clone(),
            false => panic!("{} chain api key not found", chain),
        }
    }

    pub async fn verify_tx(chain: &str, tx: &str) -> eyre::Result<bool> {
        let chainnet = Verify::get_chainnet(chain);
        let key = Verify::get_api_key(chain);
        let client = Client::new(chainnet, key).unwrap();

        let status = client.check_contract_execution_status(tx).await;
        match status {
            Ok(_) => {
                println!("Verify tx: {} status success", tx);
                return Ok(true);
            }
            Err(err) => panic!("Verify tx: {} failed, err: {:?}", tx, err),
        }
    }
}

#[tokio::test]
#[ignore = "maybe failed due to China gov firewall"]
async fn test_verify_success() {
    let res = Verify::verify_tx(
        "rinkeby",
        "0xc6e08d3b5b1077f4662907fa547fab34bac033a0501655aca0b903057c118da8",
    )
    .await
    .unwrap();
    assert!(res);
}

#[tokio::test]
#[should_panic]
// #[ignore = "maybe failed due to China gov firewall"]
async fn test_verify_failed() {
    Verify::verify_tx("rinkeby", "0xxxxxxxxxx").await.unwrap();
}
