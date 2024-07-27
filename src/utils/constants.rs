use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref SECRET: String = set_secret();
    pub static ref MAX_FILE_SIZE: u64 = set_max_file_size();
}

fn set_address() -> String {
    dotenv().ok();
    env::var("ADDRESS").unwrap_or(String::from("127.0.0.1"))
}

fn set_port() -> u16 {
    dotenv().ok();
    env::var("PORT")
        .unwrap_or_else(|_| String::from("5050"))
        .parse::<u16>()
        .expect("Cannot parse the port")
}

fn set_max_file_size() -> u64 {
    dotenv().ok();
    env::var("MAX_FILE_SIZE")
        .unwrap_or_else(|_| String::from("10485760"))
        .parse::<u64>()
        .expect("Cannot parse the port")
}

fn set_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("postgres://postgres:5862468@localhost:5432/BlogDB")
}

fn set_secret() -> String {
    dotenv().ok();
    env::var("SECRET").unwrap_or(String::from("dipeshpaudel"))
}
