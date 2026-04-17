use std::fs;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use serde::{Deserialize, Serialize};
use log::{info, error};
use reqwest::Client;
use tauri::State;
use crate::AppState;
use crate::config::{AIProvider, WhisperConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub language: Option<String>,
    pub segments: Vec<TranscriptionSegment>,
    pub duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionRequest {
    pub audio_path: String,
    pub provider: AIProvider,
    pub model: Option<String>,
    pub language: Option<String>,
}

/// 使用 Whisper API 转写音频文件
#[tauri::command]
pub async fn transcribe_audio_file(
    audio_path: String,
    state: State<'_, AppState>,
) -> Result<TranscriptionResult, String> {
    info!("开始转写音频文件: {}", audio_path);
    
    let config = state.config.lock().unwrap().clone();
    
    let audio_data = fs::read(&audio_path)
        .map_err(|e| format!("无法读取音频文件: {}", e))?;
    
    let audio_base64 = BASE64.encode(&audio_data);
    let audio_extension = std::path::Path::new(&audio_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("wav")
        .to_lowercase();
    
    let mime_type = match audio_extension.as_str() {
        "mp3" => "audio/mp3",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "m4a" => "audio/mp4",
        _ => "audio/wav",
    };
    
    let client = Client::new();
    
    let (endpoint, api_key, model) = match &config.whisper_config {
        WhisperConfig::OpenAI(whisper) => (
            format!("{}/audio/transcriptions", whisper.api_base.trim_end_matches('/')),
            whisper.api_key.clone(),
            whisper.model.clone(),
        )
        WhisperConfig::Custom(custom) => (
            format!("{}/audio/transcriptions", custom.api_base.trim_end_matches('/')),
            custom.api_key.clone(),
            custom.model.clone(),
        )
    };
    
    info!("使用 Whisper API 端点: {}", endpoint);
    
    let response = client
        .post(&endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "multipart/form-data")
        .part("file", reqwest::multipart::Part::bytes(audio_data)
            .file_name(format!("audio.{}", audio_extension))
            .mime_str(mime_type)
            .map_err(|e| format!("无法创建 multipart 部分: {}", e))?)
        .part("model", reqwest::multipart::Part::text(model))
        .part("response_format", reqwest::multipart::Part::text("verbose_json"))
        .part("timestamp_granularities[]", reqwest::multipart::Part::text("segment"))
        .send()
        .await
        .map_err(|e| format!("API 请求失败: {}", e))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        error!("Whisper API 返回错误: {}", error_text);
        return Err(format!("转写失败: {}", error_text));
    }
    
    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;
    
    info!("转写成功");
    
    let text = result["text"].as_str().unwrap_or("").to_string();
    let language = result["language"].as_str().map(|s| s.to_string());
    let duration = result["duration"].as_f64().unwrap_or(0.0);
    
    let mut segments = Vec::new();
    if let Some(segments_arr) = result["segments"].as_array() {
        for seg in segments_arr {
            segments.push(TranscriptionSegment {
                start: seg["start"].as_f64().unwrap_or(0.0),
                end: seg["end"].as_f64().unwrap_or(0.0),
                text: seg["text"].as_str().unwrap_or("").to_string(),
            });
        }
    }
    
    Ok(TranscriptionResult {
        text,
        language,
        segments,
        duration,
    })
}

/// 转写音频
#[tauri::command]
pub async fn transcribe_audio(
    audio_path: String,
    provider: AIProvider,
    model: Option<String>,
    language: Option<String>,
) -> Result<TranscriptionResult, String> {
    info!("使用提供方 {:?} 转写音频", provider);
    transcribe_audio_file(audio_path, tauri::State::default()).await
}
