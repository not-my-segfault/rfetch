mod os;
mod cpu;
mod mem;
mod user;
mod disk;
mod up;
use crate::{os::os, cpu::cpu, mem::mem, user::user, disk::disk, up::up};

fn info() -> String {
    let os = os();
    let cpu = cpu();
    let mem = mem();
    let disk = disk();
    let up = up();
    let output = format!("{}\n{}\n{}\n{}\n{}", os, cpu, mem, disk, up);
    
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