use std::process::{Command, Stdio};

pub fn up() -> String {
    let uptime = Command::new("uptime")
                          .arg("-p")
                          .stdout(Stdio::piped())
                          .output()
                          .expect("n/a");
    let uptstr = String::from_utf8_lossy(&uptime.stdout);
    let output = format!("up:    {}", uptstr.replace("up", "").replace('\n', ""));
    output
}