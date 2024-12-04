use alloy::{
    sol, // Macro for defining Solidity contracts in Rust
    primitives::{Address, U256}, // Basic Ethereum types: Address and U256
    providers::{Provider, ProviderBuilder}, // Provider to connect to the Ethereum network
    signers::local::PrivateKeySigner, // Signer to sign transactions using a private key
};
use eyre::Result; // For error handling with a convenient Result type
use std::str::FromStr; // For converting strings into other types

// Define the Solidity contract using the Alloy `sol!` macro
// This generates Rust bindings for the VoteChain contract
sol! {
    contract VoteChain {
        // Function to create a new poll in the contract
        function createPoll(
            string memory _name,           // Name of the poll
            string memory _description,    // Description of the poll
            string[] memory _options,      // Array of options for voting
            uint256 _startTime,            // Poll start time (UNIX timestamp)
            uint256 _endTime               // Poll end time (UNIX timestamp)
        ) external;

        // Function to cast a vote in a specific poll
        function castVote(uint256 pollId, string memory option) external;

        // Function to get the winner of a specific poll
        function getWinner(uint256 pollId) external view returns (string memory);
    }
}

// Struct to manage the VoteChain client
pub struct VoteChainClient {
    provider: Provider,          // HTTP provider to interact with Ethereum
    contract: VoteChain,         // Instance of the VoteChain contract
    signer: PrivateKeySigner,    // Signer for signing transactions
}

impl VoteChainClient {
    /// Creates a new instance of the VoteChain client
    /// - `rpc_url`: The URL of the Ethereum node (e.g., Infura or Alchemy)
    /// - `private_key`: The private key for signing transactions
    /// - `contract_address`: The Ethereum address of the deployed VoteChain contract
    pub fn new(
        rpc_url: &str,
        private_key: &str,
        contract_address: &str,
    ) -> Result<Self> {
        // Set up the HTTP provider to interact with the Ethereum node
        let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);

        // Configure the signer with the provided private key
        let signer = PrivateKeySigner::new(hex::decode(private_key)?)?;

        // Convert the contract address from a string to an Address type
        let contract_address = Address::from_str(contract_address)?;

        // Create an instance of the VoteChain contract
        let contract = VoteChain::new(contract_address);

        Ok(Self {
            provider,
            contract,
            signer,
        })
    }

    /// Creates a new poll on the blockchain
    /// - `name`: The name of the poll
    /// - `description`: The description of the poll
    /// - `options`: The voting options
    /// - `start_time`: The start time of the poll (UNIX timestamp)
    /// - `end_time`: The end time of the poll (UNIX timestamp)
    pub async fn create_poll(
        &self,
        name: String,
        description: String,
        options: Vec<String>,
        start_time: u64,
        end_time: u64,
    ) -> Result<()> {
        // Prepare the transaction data by encoding the createPoll function
        let tx = self
            .contract
            .createPoll(name, description, options, start_time.into(), end_time.into())
            .encode();

        // Send the transaction to the blockchain
        self.send_transaction(tx).await?;
        Ok(())
    }

    /// Casts a vote in a specific poll
    /// - `poll_id`: The ID of the poll
    /// - `option`: The chosen voting option
    pub async fn cast_vote(&self, poll_id: u64, option: String) -> Result<()> {
        // Prepare the transaction data by encoding the castVote function
        let tx = self.contract.castVote(poll_id.into(), option).encode();

        // Send the transaction to the blockchain
        self.send_transaction(tx).await?;
        Ok(())
    }

    /// Retrieves the winner of a specific poll
    /// - `poll_id`: The ID of the poll
    /// - Returns the winner as a string
    pub async fn get_winner(&self, poll_id: u64) -> Result<String> {
        // Call the getWinner function to retrieve the winner
        let result = self.contract.getWinner(poll_id.into()).call().await?;
        Ok(result)
    }

    /// Sends a signed transaction to the blockchain
    /// - `tx_data`: The raw transaction data
    async fn send_transaction(&self, tx_data: Vec<u8>) -> Result<()> {
        // Sign the transaction using the private key
        let signed_tx = self.signer.sign(&tx_data)?;

        // Send the signed transaction to the Ethereum network
        self.provider.send_raw_transaction(signed_tx).await?;
        Ok(())
    }
}
