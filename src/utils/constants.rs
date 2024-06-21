use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
}

fn set_address() -> String {
    dotenv().ok();
    env::var("ADDRESS").unwrap()
}

fn set_port() -> u16 {
    env::var("PORT").unwrap().parse::<u16>().unwrap()
}
