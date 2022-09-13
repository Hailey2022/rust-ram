use sysinfo::{System, SystemExt};
use std::{thread, time};

fn main() {
    let percent = 60 + 1;
    loop {
        let mut sys = System::new_all();
        sys.refresh_all();
        let total = sys.total_memory();
        let used = sys.used_memory();
        if total / 100 * percent > used {
            let ram_to_use: u64 = total / 100 * percent - used;
            println!("add: {} bytes", ram_to_use);
            let _ram = vec![1 as u128; (ram_to_use / 16) as usize];
            loop {
                sys.refresh_all();
                let total = sys.total_memory();
                let used = sys.used_memory();
                println!("total: {} bytes, used: {} bytes", total, used);
                if total / 100 * (percent - 5) < used && used < total / 100 * (percent + 5) {
                    let time = time::Duration::from_secs(5);
                    thread::sleep(time);
                } else {
                    break;
                }
            }
        } else {
            let time = time::Duration::from_secs(10);
            thread::sleep(time);
        }
    }
}
