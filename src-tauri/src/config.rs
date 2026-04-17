use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use log::{info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub ai_service: Option<AIServiceConfig>,
    pub whisper_config: WhisperConfig,
    pub recording_config: RecordingSettings,
    pub general_config: GeneralSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIServiceConfig {
    pub provider: String,
    pub api_base: String,
    pub api_key: String,
    pub model: String,
    pub timeout_seconds: u32,
    pub use_proxy: bool,
    pub proxy_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhisperConfig {
    OpenAI(OpenAIWhisperConfig),
    Custom(CustomWhisperConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIWhisperConfig {
    pub api_base: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomWhisperConfig {
    pub api_base: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingSettings {
    pub default_sample_rate: u32,
    pub default_channels: u16,
    pub auto_noise_reduction: bool,
    pub enable_vad: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub language: String,
    pub theme: String,
    pub auto_copy_summary: bool,
    pub notification_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Google,
    Qwen,
    Zhipu,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ai_service: None,
            whisper_config: WhisperConfig::OpenAI(OpenAIWhisperConfig {
                api_base: "https://api.openai.com/v1".to_string(),
                api_key: String::new(),
                model: "whisper-1".to_string(),
            }),
            recording_config: RecordingSettings {
                default_sample_rate: 44100,
                default_channels: 1,
                auto_noise_reduction: true,
                enable_vad: true,
            },
            general_config: GeneralSettings {
                language: "zh-CN".to_string(),
                theme: "light".to_string(),
                auto_copy_summary: false,
                notification_enabled: true,
            },
        }
    }
}

impl Default for RecordingSettings {
    fn default() -> Self {
        Self {
            default_sample_rate: 44100,
            default_channels: 1,
            auto_noise_reduction: true,
            enable_vad: true,
        }
    }
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            language: "zh-CN".to_string(),
            theme: "light".to_string(),
            auto_copy_summary: false,
            notification_enabled: true,
        }
    }
}

#[tauri::command]
pub fn get_config_path() -> String {
    get_config_file_path().to_string_lossy().to_string()
}

fn get_config_file_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("meeting-recorder");
    config_dir.join("config.json")
}

pub fn load_config_from_file() -> Result<AppConfig, String> {
    let path = get_config_file_path();
    
    if !path.exists() {
        info!("配置文件不存在，使用默认配置");
        return Ok(AppConfig::default());
    }
    
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("无法读取配置文件: {}", e))?;
    
    serde_json::from_str(&content)
        .map_err(|e| format!("无法解析配置文件: {}", e))
}

#[tauri::command]
pub fn save_config(
    config: AppConfig,
    state: tauri::State<'_, crate::AppState>,
) -> Result<(), String> {
    let path = get_config_file_path();
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("无法创建配置目录: {}", e))?;
    }
    
    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("无法序列化配置: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("无法写入配置文件: {}", e))?;
    
    *state.config.lock().unwrap() = config;
    
    info!("配置已保存到: {}", path.display());
    
    Ok(())
}

#[tauri::command]
pub fn load_config(
    state: tauri::State<'_, crate::AppState>,
) -> AppConfig {
    state.config.lock().unwrap().clone()
}
