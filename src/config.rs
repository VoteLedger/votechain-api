use dotenv::dotenv;
use std::env;

const REQUIRED_KEYS: [&str; 3] = [
    "BLOCKCHAIN_NODE_URL",
    "CONTRACT_ADDRESS",
    "JWT_SECRET",
];

pub fn load_env() -> bool {
    dotenv().ok();

    REQUIRED_KEYS.iter().all(|key| {
        if env::var(key).is_ok() {
            true
        } else {
            eprintln!("Missing required environment variable: {}", key);
            false
        }
    })
}
