use std::env;

#[allow(non_snake_case)]
pub struct Config {
    pub POSTGRES_USER: String,
    pub POSTGRES_PASSWORD: String,
    pub POSTGRES_HOST: String,
    pub POSTGRES_PORT: String,
    pub POSTGRES_DB: String,
}

impl Config {
    #[allow(non_snake_case)]
    pub fn init() -> Self {
        let POSTGRES_USER = get_env("POSTGRES_USER");
        let POSTGRES_PASSWORD = get_env("POSTGRES_PASSWORD");
        let POSTGRES_HOST = get_env("POSTGRES_HOST");
        let POSTGRES_PORT = get_env("POSTGRES_PORT");
        let POSTGRES_DB = get_env("POSTGRES_DB");

        Config {
            POSTGRES_USER,
            POSTGRES_PASSWORD,
            POSTGRES_HOST,
            POSTGRES_PORT,
            POSTGRES_DB,
        }
    }
}

pub fn get_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        panic!("{} must be set", name);
    })
}
