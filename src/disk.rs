use sys_info::disk_info;

pub fn disk() -> String {
    let info = disk_info().unwrap();
    let total = info.total;
    let free = info.free;
    let used = total as f32 - free as f32;
    let output = format!("disk:   {:.2} GB / {:.2} GB total", used as f32 / 1000000 as f32, total as f32 / 1000000 as f32);
    output
}