use sys_info::hostname;

pub fn user() -> String {
    let user = whoami::username();
    let host = hostname().unwrap();
    let output = format!("{}@{}", user, host);
    output
}