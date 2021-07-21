use sys_info::hostname;
use std::env;

pub fn user() -> String {
    let user = env::var("USER").unwrap();
    let host = hostname().unwrap();

    let output = format!("{}@{}", user, host);

    output
}