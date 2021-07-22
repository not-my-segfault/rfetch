use sys_info::{linux_os_release, os_release, os_type};

pub fn os() -> String {
    if os_type().unwrap() == "Linux" {
        let release = linux_os_release().unwrap();
        let osname = format!("{} {} {}", release.name.unwrap_or("".to_string()), release.version.unwrap_or("".to_string()), release.version_codename.unwrap_or("".to_string()));
        let kernel = os_release().unwrap();
        let output = format!("os:     {}\nkernel: {}", osname, kernel);
        output
    } else {
        let release = os_type().unwrap();
        let osname = format!("{}", release);
        let kernel = os_release().unwrap();
        let output = format!("os:     {}\nkernel: {}", osname, kernel);
        output
    }
}