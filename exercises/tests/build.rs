//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前UNIX时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置编译时环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 如果要确保测试期间环境变量可用，可以额外添加（非必须）
    println!("cargo:rerun-if-env-changed=TEST_FOO");
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
