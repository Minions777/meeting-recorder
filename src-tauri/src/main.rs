#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use meeting_recorder_lib::{run, setup_logging};

fn main() {
    // 初始化日志系统
    setup_logging();
    
    // 运行应用
    run();
}
