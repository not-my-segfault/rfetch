use std::process::{Command, Stdio};
use sys_info::os_type;

pub fn up() -> String {
    if os_type().unwrap() != "Linux" {
    let output = "up:     unsupported"; // ill find a better way one day
    output.to_string()
    } else {
    let uptime = Command::new("uptime")
                          .arg("-p")
                          .stdout(Stdio::piped())
                          .output()
                          .expect("n/a");
    let uptstr = String::from_utf8_lossy(&uptime.stdout);
    let output = format!("up:    {}", uptstr.replace("up", "").replace('\n', ""));
    output
    }
}