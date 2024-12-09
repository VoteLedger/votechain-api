use alloy::primitives::{address, ruint::aliases::U256};
use log::info;
use serde::Serialize;

use crate::{VotechainContractInstance, VOTECHAIN::pollsReturn};

pub struct VotechainContract {
    contract: VotechainContractInstance,
}

#[derive(Serialize)]
pub struct Poll {
    pub name: String,
    pub description: String,
    // pub options: Vec<String>,
    pub start_time: U256,
    pub end_time: U256,
    pub winner: String,
    pub is_ended: bool,
}

// Implement conversion from `pollsReturn` to `Poll`
impl From<pollsReturn> for Poll {
    fn from(poll: pollsReturn) -> Self {
        Self {
            name: poll.name,
            description: poll.description,
            // options: poll.options,
            start_time: poll.start_time,
            end_time: poll.end_time,
            winner: poll.winner,
            is_ended: poll.is_ended,
        }
    }
}

#[derive(Serialize)]
pub struct PollRecipt {
    poll_id: U256,
}

impl VotechainContract {
    pub fn new(instance: VotechainContractInstance) -> Self {
        // Create a new instance of the contract
        Self { contract: instance }
    }

    pub async fn create_poll(
        &self,
        name: String,
        description: String,
        options: Vec<String>,
        start_time: U256,
        end_time: U256,
    ) -> Result<PollRecipt, String> {
        // Print some debug info about the poll
        info!(
            "Poll data: {:?}",
            (
                name.clone(),
                description.clone(),
                options.clone(),
                start_time,
                end_time
            )
        );

        info!("Creating poll: {}", name);
        let poll_id = self
            .contract
            .create_poll(name, description, options, start_time, end_time)
            .call()
            .await
            .expect("Failed to create poll")
            ._0;
        info!("Resulting ID: {}", poll_id.clone());

        // .into_transaction_request();
        // Use our provider to send the transaction
        //
        // // Get wallet information from the config file
        // let srv_wallet_key = std::env::var("RELAY_WALLET_PRIVATE_KEY").unwrap();
        //
        // // Create signer based on private key
        // let signer: PrivateKeySigner = srv_wallet_key.parse().expect("Invalid private key");
        // let wallet = EthereumWallet::new(signer);
        //
        // // create link with available contracts + connect to blockchain
        // let rpc_url = Url::parse(&std::env::var("RPC_URL").unwrap()).unwrap();
        //
        // // create provider + link the wallet for signing
        // let provider = ProviderBuilder::new()
        //     .with_recommended_fillers()
        //     .wallet(wallet)
        //     .on_http(rpc_url);
        //
        // let tx = provider.fill(tx).await.expect("Failed to fill transaction");
        //
        // let result = provider
        //     .send_tx_envelope(tx.as_envelope().expect("Failed to create envelope").clone())
        //     .await;

        Ok(PollRecipt {
            poll_id, // transaction_hash: tx_address.to_string(),
        })
    }

    pub async fn get_poll(&self, id: U256) -> Result<Poll, String> {
        let poll = self
            .contract
            .polls(id)
            .call()
            .await
            .expect("Failed to get poll");
        Ok(poll.into())
    }

    pub async fn cast_vote(&self, poll_id: U256, option: String) -> Result<(), String> {
        let result = self.contract.cast_vote(poll_id, option).call().await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_available_polls(&self) -> Result<Vec<Poll>, String> {
        // Get total number of polls
        let wrapped_count = self
            .contract
            .poll_count()
            .call()
            .await
            .expect("Failed to get poll count")
            ._0;

        info!("Wrapped count: {:?}", wrapped_count);

        // convert to integer number
        let count: u64 = wrapped_count
            .try_into()
            .expect("Failed to convert U256 to integer. Value is too large!");

        info!("Total number of polls: {}", count);

        // Initialize vector to store poll data
        let mut polls = Vec::new();

        // Iterate over all available polls
        for i in 0..count {
            // Get poll data
            let poll = self
                .contract
                .polls(i.try_into().expect("Invaid cast"))
                .call()
                .await;
            // Check if poll data is valid
            match poll {
                Ok(poll) => {
                    // Append poll data to vector
                    polls.push(poll.into());
                }
                Err(e) => {
                    // Return error if poll data is invalid
                    return Err(e.to_string());
                }
            }
        }

        // Return vector of poll data
        Ok(polls)
    }
}
