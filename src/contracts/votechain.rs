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
            .createPoll(name, description, options, start_time, end_time)
            .call()
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn get_polls(
        &self,
    ) -> Result<Vec<(String, String, Vec<String>, U256, U256)>, String> {
        // Compute current time in seconds

        let result = self.contract.polls().call().await;
        match result {
            Ok(polls) => Ok(polls),
            Err(e) => Err(e.to_string()),
        }
    }
}
