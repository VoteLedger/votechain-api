use alloy::{
    primitives::{ruint::aliases::U256, Address},
    providers::RootProvider,
    sol,
    transports::http::{Client, Http},
};
use VOTECHAIN::VOTECHAINInstance;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    VOTECHAIN,
    "contracts/abi/votechain.json"
);

pub struct VotechainContract {
    contract: VOTECHAINInstance<Http<Client>, RootProvider<Http<Client>>>,
}

impl VotechainContract {
    pub fn new(address: Address, provider: RootProvider<Http<Client>>) -> Self {
        // Create a new instance of the contract
        Self {
            contract: VOTECHAIN::new(address, provider),
        }
    }

    pub async fn create_poll(
        &self,
        name: String,
        description: String,
        options: Vec<String>,
        start_time: U256,
        end_time: U256,
    ) -> Result<(), String> {
        let result = self
            .contract
            .create_poll(name, description, options, start_time, end_time)
            .call()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_available_polls(&self) -> Result<Vec<VOTECHAIN::pollsReturn>, String> {
        // Get total number of polls
        let wrapped_count = self
            .contract
            .poll_count()
            .call()
            .await
            .expect("Failed to get poll count")
            ._0;

        // convert to integer number
        let count: u64 = wrapped_count
            .try_into()
            .expect("Failed to convert U256 to integer. Value is too large!");

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
                    polls.push(poll);
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
