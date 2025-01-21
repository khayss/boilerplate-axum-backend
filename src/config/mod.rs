use std::env;

#[allow(non_snake_case)]
pub struct Config {
    pub DB_USERNAME: String,
    pub DB_PASSWORD: String,
    pub DB_HOST: String,
    pub DB_PORT: String,
    pub DB_NAME: String,
}

impl Config {
    #[allow(non_snake_case)]
    pub fn init() -> Self {
        let DB_USERNAME = get_env("DB_USERNAME");
        let DB_PASSWORD = get_env("DB_PASSWORD");
        let DB_HOST = get_env("DB_HOST");
        let DB_PORT = get_env("DB_PORT");
        let DB_NAME = get_env("DB_NAME");

        Config {
            DB_USERNAME,
            DB_PASSWORD,
            DB_HOST,
            DB_PORT,
            DB_NAME,
        }
    }
}

pub fn get_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| {
        panic!("{} must be set", name);
    })
}
