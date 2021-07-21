mod os;
mod cpu;
mod mem;
mod user;
mod disk;
use crate::{os::os, cpu::cpu, mem::mem, user::user, disk::disk};

fn info() -> String {
    let os = os();
    let cpu = cpu();
    let mem = mem();
    let disk = disk();
    let output = format!("{}\n{}\n{}\n{}", os, cpu, mem, disk);
    
    output
}

fn main() {

    let user = user();
    let info = info();

    let topstr = format!("=== {} ===", user);
    let topstrlen = topstr.chars().count();
    let botstr = format!("{}", "=".repeat(topstrlen));
    println!("{}", topstr);
    println!("{}", info);
    println!("{}", botstr);
}