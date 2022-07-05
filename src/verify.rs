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

        let status = client.check_transaction_receipt_status(tx).await;
        match status {
            Ok(_) => {
                println!("Verify tx: {} status success", tx);
                return Ok(true);
            }
            Err(err) => {
                println!("Verify tx: {} failed, err: {:?}", tx, err);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    // #[ignore = "maybe failed due to China gov firewall"]
    async fn test_rinkeby_verify() {
        let chain = "rinkeby";
        let existed_tx = "0xc6e08d3b5b1077f4662907fa547fab34bac033a0501655aca0b903057c118da8";
        let res = Verify::verify_tx(chain, existed_tx).await.unwrap();
        assert!(res);

        let not_existed_tx = "0xabcabcababcabcababcabcababcabcababcabcababcabcababcabcababcabcab";
        let res = Verify::verify_tx(chain, not_existed_tx).await.unwrap();
        assert!(!res);
    }

    #[tokio::test]
    // #[ignore = "maybe failed due to China gov firewall"]
    async fn test_fantom_verify() {
        let chain = "fantom";
        let existed_tx = "0xb4b8b03c36ff4d6668c7aab2c78a4936b3ac79dda5e07cf7e509c01680fea443";
        let res = Verify::verify_tx(chain, existed_tx).await.unwrap();
        assert!(res);

        let not_existed_tx = "0xabcabcababcabcababcabcababcabcababcabcababcabcababcabcababcabcab";
        let res = Verify::verify_tx(chain, not_existed_tx).await.unwrap();
        assert!(!res);
    }

    #[tokio::test]
    // #[ignore = "maybe failed due to China gov firewall"]
    async fn test_polygon_verify() {
        let chain = "polygon";
        let existed_tx = "0x803aa2410fb9976c432e5390728798c98a8b4dda4ef694e7dab79f25cdffcdd6";
        let res = Verify::verify_tx(chain, existed_tx).await.unwrap();
        assert!(res);

        let not_existed_tx = "0xabcabcababcabcababcabcababcabcababcabcababcabcababcabcababcabcab";
        let res = Verify::verify_tx(chain, not_existed_tx).await.unwrap();
        assert!(!res);
    }

    #[tokio::test]
    // #[ignore = "maybe failed due to China gov firewall"]
    async fn test_bsc_verify() {
        let chain = "bsc";
        let existed_tx = "0x98821751920196f1c5919635b7c371af7adfac2c6f7be2d832aae39f303b2406";
        let res = Verify::verify_tx(chain, existed_tx).await.unwrap();
        assert!(res);

        let not_existed_tx = "0xabcabcababcabcababcabcababcabcababcabcababcabcababcabcababcabcab";
        let res = Verify::verify_tx(chain, not_existed_tx).await.unwrap();
        assert!(!res);
    }

    #[tokio::test]
    // #[ignore = "maybe failed due to China gov firewall"]
    async fn test_arbitrum_verify() {
        let chain = "arbitrum";
        let existed_tx = "0xc75a5d7ffccd3b8fc00a124a683b7e768e1a4d6a17a5977ed931965886109bf5";
        let res = Verify::verify_tx(chain, existed_tx).await.unwrap();
        assert!(res);

        let not_existed_tx = "0xabcabcababcabcababcabcababcabcababcabcababcabcababcabcababcabcab";
        let res = Verify::verify_tx(chain, not_existed_tx).await.unwrap();
        assert!(!res);
    }

    #[tokio::test]
    // #[ignore = "maybe failed due to China gov firewall"]
    async fn test_optimism_verify() {
        let chain = "optimism";
        let existed_tx = "0x319c40f66639e3f1a5954621f93ad19b78043d0d8a5bbf65e20fa7f8929afd03";
        let res = Verify::verify_tx(chain, existed_tx).await.unwrap();
        assert!(res);

        let not_existed_tx = "0xabcabcababcabcababcabcababcabcababcabcababcabcababcabcababcabcab";
        let res = Verify::verify_tx(chain, not_existed_tx).await.unwrap();
        assert!(!res);
    }

    #[tokio::test]
    // #[ignore = "maybe failed due to China gov firewall"]
    async fn test_avalanche_verify() {
        let chain = "avalanche";
        let existed_tx = "0x4a9eeaeef2990af4aa915e1433e28bf6f45b04b96f29c3c09cf6f1b79f5bfd5c";
        let res = Verify::verify_tx(chain, existed_tx).await.unwrap();
        assert!(res);

        let not_existed_tx = "0xabcabcababcabcababcabcababcabcababcabcababcabcababcabcababcabcab";
        let res = Verify::verify_tx(chain, not_existed_tx).await.unwrap();
        assert!(!res);
    }
}
