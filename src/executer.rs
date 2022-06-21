
use std::option;

use crate::utils::*;
use crate::config::Config;
use ethers::prelude::SignerMiddleware;

pub struct Executer {
    pub cfg: Config,
    // verifier: Verifier,
}

impl<'a> Executer {
    pub fn new() -> Self {
        Self {
            cfg: Config::new().unwrap(),
            // verifier: None,
        }
    }

    pub async fn run(self) -> eyre::Result<()> {
        let provider = get_http_provider(
            &self.cfg.rpc_url.unwrap_or_else(|| "http://localhost:8545".to_string()),
            false,
        );
        let wallet = get_from_private_key(&self.cfg.pri_key.unwrap_or_else(|| "".to_string()));
        let provider = SignerMiddleware::new(provider.clone(), wallet.unwrap());
        
        for mut contract in self.cfg.contracts {
            contract.run(provider.clone()).await?;
        }
        Ok(())
    }
}