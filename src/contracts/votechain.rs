use alloy::{
    primitives::{address, Address},
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

    pub async fn create_poll(&self, private_key: &str, question: String) -> Result<(), String> {
        Ok(())
    }
}
