use alloy::contract::{Contract, Function};
use alloy::providers::{Provider, ProviderBuilder};
use std::sync::Arc;

pub struct ContractService {
    provider: Arc<Provider>,
    contract: Contract,
}

impl ContractService {
    pub async fn new(rpc_url: &str, contract_address: &str, abi: &[u8]) -> Self {
        let provider = Arc::new(ProviderBuilder::new().on_http(rpc_url.parse().unwrap()));
        let contract = Contract::from_abi(contract_address.parse().unwrap(), abi.to_vec());

        ContractService {
            provider,
            contract,
        }
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
                println!("New Poll Created: ID: {}, Question: {}", decoded.0, decoded.1);
            }
        }
    }
}
