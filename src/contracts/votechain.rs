use alloy::{
    primitives::{ruint::aliases::U256, Address},
    providers::RootProvider,
    sol,
    transports::http::{Client, Http},
};
use serde::Serialize;
use VOTECHAIN::{pollsReturn, VOTECHAINInstance};

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

    pub async fn get_poll(&self, id: U256) -> Result<Poll, String> {
        let poll = self
            .contract
            .polls(id)
            .call()
            .await
            .expect("Failed to get poll");
        Ok(poll.into())
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
