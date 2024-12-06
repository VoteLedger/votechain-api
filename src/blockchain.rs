use alloy::contract::Contract;
use alloy::providers::{Http, Provider, ProviderBuilder};
//use alloy::signers::{LocalWallet, Signer};
use alloy::signers::local::PrivateKeySigner;
use std::env;
use std::sync::Arc;

pub struct BlockchainManager {
    provider: Arc<Provider<Http>>,
    contract: Contract<Arc<Provider<Http>>>,
}

impl BlockchainManager {
    pub async fn new() -> Self {
        let rpc_url = env::var("RPC_URL").unwrap();
        let private_key = env::var("PRIVATE_KEY").unwrap();
        let contract_address =
            env::var("CONTRACT_ADDRESS").unwrap();
        let abi_path = "abi/VoteChain.json";

        // Setup provider
       // let provider = ProviderBuilder::new().on_http(rpc_url);
       // let provider = Arc::new(provider);

        // Setup wallet
        //let wallet: LocalWallet = private_key.parse().unwrap();
        //let client = wallet.connect(provider.clone());

        // Following wallet and provider definitions are based on what found on the documentation
        // They might be wrong, i don't trust documentations so we'll see

        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = EthereumWallet::from(signer);

        // Create a provider with the wallet.
        let rpc_url = anvil.endpoint_url();
        let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(rpc_url);

        // Load contract ABI
        let abi = std::fs::read_to_string(abi_path).unwrap();
        let contract = Contract::from_json(
            contract_address.parse().unwrap(),
            abi,
            client,
        )
        .unwrap();

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