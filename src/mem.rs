use sys_info::mem_info;

pub fn mem() -> String {
    let info = mem_info().unwrap();
    let total = info.total;
    let avail = info.avail;
    let used = total as f32 - avail as f32;
    let output = format!("mem:    {:.2} GB / {:.2} GB total", used as f32 / 1000000 as f32, total as f32 / 1000000 as f32);
    output
}