use sys_info::{linux_os_release, os_release};

pub fn os() -> String{
    let release = linux_os_release().unwrap();
    let osname = format!("{} {} {}", release.name.unwrap(), release.version.unwrap_or("".to_string()), release.version_codename.unwrap_or("".to_string())).to_lowercase();
    let kernel = os_release().unwrap();
    let output = format!("os:     {}\nkernel: {}", osname, kernel);

    output
}