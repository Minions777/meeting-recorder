mod audio;
mod transcription;
mod ai_service;
mod config;
mod utils;

use std::sync::Mutex;
use log::{info};

// 全局状态管理
pub struct AppState {
    pub recording: Mutex<bool>,
    pub current_audio_path: Mutex<Option<String>>,
    pub config: Mutex<config::AppConfig>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            recording: Mutex::new(false),
            current_audio_path: Mutex::new(None),
            config: Mutex::new(config::AppConfig::default()),
        }
    }
}

/// 初始化日志系统
pub fn setup_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
    info!("会议记录助手启动中...");
}

/// 运行 Tauri 应用
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            // 录音相关命令
            audio::start_recording,
            audio::stop_recording,
            audio::get_recording_status,
            audio::get_audio_devices,
            audio::set_audio_device,
            // 转写相关命令
            transcription::transcribe_audio,
            transcription::transcribe_audio_file,
            // AI 服务相关命令
            ai_service::summarize_text,
            ai_service::test_connection,
            ai_service::get_available_models,
            // 配置相关命令
            config::load_config,
            config::save_config,
            config::get_config_path,
            // 工具命令
            utils::copy_to_clipboard,
            utils::export_to_file,
            utils::get_app_info,
        ])
        .setup(|app| {
            info!("应用初始化完成");
            
            // 加载配置
            let state = app.state::<AppState>();
            if let Ok(config) = config::load_config_from_file() {
                *state.config.lock().unwrap() = config;
                info!("配置加载成功");
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动应用时发生错误");
}
