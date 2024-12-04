use dotenv::from_path;
use std::env;

const KEYS: [&str; 6] = [
    "POSTGRES_HOST",
    "POSTGRES_PORT",
    "POSTGRES_USER",
    "POSTGRES_PASSWORD",
    "POSTGRES_DB",
    "DATABASE_URL",
];

pub fn load_env() -> bool {
    let root_dir = env::current_dir().expect("Failed to get current directory");
    let dotfile = root_dir.join(".env");

    // Load the .env file from the specified path
    if let Err(e) = from_path(&dotfile) {
        println!("[!] Failed to load .env file: {}", e);
        return false;
    }

    // Ensure all keys are present
    let mut ok = true;
    for key in KEYS {
        match env::var(key) {
            Ok(_v) => {}
            Err(_e) => {
                println!("[!] Config {} not present.", key);
                ok = false;
            }
        }
    }

    ok
}
