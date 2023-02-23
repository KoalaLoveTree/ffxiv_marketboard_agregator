use dotenv::dotenv;
use std::collections::HashMap;

pub fn parse_env_variables() -> HashMap<String, String> {
    dotenv().ok();

    let mut config: HashMap<String, String> = HashMap::new();

    config.insert(
        "DATABASE_HOST".to_string(),
        std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set!"),
    );
    config.insert(
        "DATABASE_NAME".to_string(),
        std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set!"),
    );
    config.insert(
        "DATABASE_PASSWORD".to_string(),
        std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set!"),
    );
    config.insert(
        "DATABASE_USERNAME".to_string(),
        std::env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set!"),
    );

    config
}
