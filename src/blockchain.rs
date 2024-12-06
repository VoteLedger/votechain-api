use ethers::providers::{Http, Provider};
use ethers::contract::Contract;
use ethers::signers::{LocalWallet, Signer};
use std::env;
use std::sync::Arc;

pub struct BlockchainManager {
    provider: Arc<Provider<Http>>,
    contract: Contract<Arc<Provider<Http>>>,
}

impl BlockchainManager {
    pub async fn new() -> Self {
        let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env file");
        let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env file");
        let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set in .env file");
        let abi_path = "abi/contract_abi.json";

        // Setup provider
        let provider = Provider::<Http>::try_from(rpc_url).expect("Failed to connect to provider");
        let provider = Arc::new(provider);

        // Setup wallet
        let wallet: LocalWallet = private_key.parse().expect("Invalid private key");
        let client = wallet.connect(provider.clone());

        // Load contract ABI
        let abi = std::fs::read_to_string(abi_path).expect("Failed to read ABI file");
        let contract = Contract::from_json(contract_address.parse().expect("Invalid contract address"), abi, client)
            .expect("Failed to create contract instance");

        BlockchainManager { provider, contract }
    }

    pub async fn get_chain_id(&self) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(self.provider.get_chain_id().await?.as_u64())
    }

    pub async fn call_get_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self
            .contract
            .method::<_, String>("getData", ())?
            .call()
            .await?)
    }
}

