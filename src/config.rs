use dotenv::dotenv;
use std::collections::HashMap;

pub fn parse_env_variables() -> HashMap<String, String> {
    dotenv().ok();

    let mut config: HashMap<String, String> = HashMap::new();

    config.insert(
        "DATABASE_URL".to_string(),
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!"),
    );

    config
}
