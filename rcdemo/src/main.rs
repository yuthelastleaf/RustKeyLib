use std::time::Duration;
use std::ffi::{c_int, c_ulonglong, c_ushort};
use std::collections::BTreeSet;
mod key_inter;

// Rust 的回调函数
extern "C" fn rust_callback(value: c_ulonglong) -> c_ushort {
    println!("Rust: Callback received value: {}", value);
    return 1;
}

// Rust 的回调函数
extern "C" fn rust_filter_callback(value: c_ulonglong) -> c_ushort {
    println!("Rust: filter Callback received value: {}", value);
    unsafe {
        if key_inter::iskeyintag(value, key_inter::KeyCode::KEY_LCONTROL as i32) {
            println!("Rust: filter Callback received Control Key");
        }

        if key_inter::iskeyintag(value, key_inter::KeyCode::KEY_O as i32) {
            println!("Rust: filter Callback received O Key");
        }
    }
    return 1;
}

fn main() {
    unsafe {
        // 注册回调函数
        // register_callback(rust_callback);
        key_inter::register_filter(keycode_set!([key_inter::KeyCode::KEY_LCONTROL, key_inter::KeyCode::KEY_O]), rust_filter_callback);
        println!("Rust: Callback registered.");

    }
    
    // 让程序持续运行 60 秒
    println!("Rust: Sleeping for 60 seconds to prevent exit...");
    std::thread::sleep(Duration::from_secs(6000));

}