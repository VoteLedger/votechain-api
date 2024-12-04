use alloy::sol;
use eyre::Result;

sol! {
    pragma solidity ^0.8.0;

    contract VoteChain {
        function castVote(uint256 pollId, string option) public;
    }
}

pub async fn cast_vote(poll_id: u32, option: &str, wallet_address: &str) -> Result<()> {
    let node_url = std::env::var("BLOCKCHAIN_NODE_URL")?;
    let client = alloy::client::Client::new(node_url).await?;

    let contract_address = std::env::var("CONTRACT_ADDRESS")?;
    let contract = VoteChain::new(contract_address.parse()?, client);

    // Esegui la chiamata alla funzione `castVote`
    contract.cast_vote(poll_id.into(), option.to_string()).send().await?;
    Ok(())
}
