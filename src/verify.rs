// contract info to verify
#[derive(Serialize, Deserialize)]
pub struct VerifyInfo {
    pub addr: Option<String>,
}

pub fn verify(contract: String) -> Result<()> {
    let resp = etherscan
        .submit_contract_verification(&verify_args)
        .await
        .wrap_err("Failed to submit contract verification")?;

    if resp.status == "0" {
        if resp.result == "Contract source code already verified" {
            return Ok(None);
        }

        if resp.result.starts_with("Unable to locate ContractCode at") {
            warn!("{}", resp.result);
            return Err(eyre!("Etherscan could not detect the deployment."));
        }

        warn!("Failed verify submission: {:?}", resp);
        eprintln!(
            "Encountered an error verifying this contract:\nResponse: `{}`\nDetails: `{}`",
            resp.message, resp.result
        );
        std::process::exit(1);
    }

    if let Some(resp) = resp {
        println!(
            "Submitted contract for verification:\n\tResponse: `{}`\n\tGUID: `{}`\n\tURL: {}",
            resp.message,
            resp.result,
            etherscan.address_url(self.address)
        );

        if self.watch {
            let check_args = VerifyCheckArgs {
                guid: resp.result,
                chain: self.chain,
                retry: RETRY_CHECK_ON_VERIFY,
                etherscan_key: self.etherscan_key,
            };
            return check_args.run().await
        }
    } else {
        println!("Contract source code already verified");
    }

    Ok(())
}
