use alloy::contract::{ContractInstance, Interface};
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder, RootProvider};
use alloy::transports::http::{reqwest, Client, Http};
use std::any::Any;
use std::sync::Arc;

pub struct ContractService {
    provider: Arc<dyn Provider>,
    contract: ContractInstance<Any, Arc<RootProvider<Http<Client>>>>,
}

impl ContractService {
    // NOTE: Follow this example to understand the new API...
    // https://github.com/alloy-rs/examples/blob/5a6776f2400312ee6beb7ee108d9407cd889c078/examples/contracts/examples/interact_with_contract_instance.rs
    pub async fn new(rpc_url: &str, contract_address: Address, abi: &str) -> Self {
        // create provider
        let url = reqwest::Url::parse(rpc_url).unwrap();
        let provider = Arc::new(ProviderBuilder::new().on_http(url));

        // convert abi to json
        let abi_json = serde_json::from_str(abi).unwrap();

        let contract =
            ContractInstance::new(contract_address, provider.clone(), Interface::new(abi));

        ContractService { provider, contract }
    }

    pub async fn get_poll(&self, poll_id: u64) -> Result<String, Box<dyn std::error::Error>> {
        let function = Function::new("getPoll", vec![poll_id.into()]);
        let result: String = self
            .contract
            .call_function(&self.provider, &function)
            .await?;
        Ok(result)
    }

    pub async fn create_poll(
        &self,
        private_key: &str,
        question: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let function = Function::new("createPoll", vec![question.into()]);
        self.contract
            .sign_and_send_transaction(&self.provider, &function, private_key)
            .await?;
        Ok(())
    }

    pub async fn listen_poll_created(&self) {
        let event = self.contract.event("PollCreated").unwrap();
        let mut stream = event.stream(&self.provider).await.unwrap();

        while let Some(log) = stream.next().await {
            if let Ok(decoded) = log.decode::<(u64, String)>() {
                println!(
                    "New Poll Created: ID: {}, Question: {}",
                    decoded.0, decoded.1
                );
            }
        }
    }
}
