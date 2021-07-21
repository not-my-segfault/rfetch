use sys_info::{cpu_num, cpu_speed};

pub fn cpu() -> String {
    let corecount = cpu_num().unwrap();
    let corespeed = cpu_speed().unwrap();
    let ghz = corespeed as f32 / 1000 as f32; 
    let output = format!("cpu:    {} logical cores @ {} GHz", corecount, ghz);
    output
}