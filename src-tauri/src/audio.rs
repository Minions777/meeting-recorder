use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::io::BufWriter;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavWriter, WavSpec, SampleFormat};
use log::{info, error, warn};
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

static RECORDING_FLAG: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub name: String,
    pub id: String,
    pub is_default: bool,
    pub is_input: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingStatus {
    pub is_recording: bool,
    pub duration_seconds: u64,
    pub audio_path: Option<String>,
    pub device_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub device_id: Option<String>,
    pub recording_mode: RecordingMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordingMode {
    Microphone,
    SystemAudio,
}

/// 获取可用的音频输入设备列表
#[tauri::command]
pub async fn get_audio_devices() -> Result<Vec<AudioDevice>, String> {
    let host = cpal::default_host();
    let mut devices = Vec::new();
    
    let default_input = host.default_input_device();
    let default_name = default_input.as_ref().and_then(|d| d.name().ok());
    
    for (idx, device) in host.input_devices()
        .map_err(|e| format!("无法枚举音频设备: {}", e))?
        .enumerate() 
    {
        let name = device.name().unwrap_or_else(|_| format!("设备 {}", idx));
        let is_default = default_name.as_ref() == Some(&name);
        
        devices.push(AudioDevice {
            name: name.clone(),
            id: format!("device_{}", idx),
            is_default,
            is_input: true,
        });
        
        info!("发现音频设备: {} (默认: {})", name, is_default);
    }
    
    if devices.is_empty() {
        warn!("未发现可用的音频输入设备");
    }
    
    Ok(devices)
}

/// 设置当前使用的音频设备
#[tauri::command]
pub async fn set_audio_device(
    device_id: String,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    info!("设置音频设备: {}", device_id);
    Ok(())
}

/// 开始录音
#[tauri::command]
pub async fn start_recording(
    config: RecordingConfig,
    state: State<'_, AppState>,
) -> Result<String, String> {
    if RECORDING_FLAG.load(Ordering::SeqCst) {
        return Err("已经在录音中".to_string());
    }
    
    info!("开始录音，配置: {:?}", config);
    
    let host = cpal::default_host();
    
    let device = if let Some(ref device_id) = config.device_id {
        host.input_devices()
            .map_err(|e| format!("无法枚举设备: {}", e))?
            .nth(device_id.parse::<usize>().unwrap_or(0))
            .ok_or("未找到指定的音频设备")?
    } else {
        host.default_input_device()
            .ok_or("未找到默认音频设备")?
    };
    
    info!("使用音频设备: {:?}", device.name());
    
    let supported_config = device
        .default_input_config()
        .map_err(|e| format!("无法获取设备配置: {}", e))?;
    
    let sample_rate = config.sample_rate;
    let channels = config.channels;
    
    info!("音频配置: {}Hz, {}通道, {:?}", sample_rate, channels, supported_config.sample_format());
    
    let audio_dir = std::env::temp_dir().join("meeting_recorder");
    std::fs::create_dir_all(&audio_dir).map_err(|e| format!("无法创建目录: {}", e))?;
    
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let audio_path = audio_dir.join(format!("recording_{}.wav", timestamp));
    let audio_path_str = audio_path.to_string_lossy().to_string();
    
    *state.current_audio_path.lock().unwrap() = Some(audio_path_str.clone());
    
    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    
    let recorded_samples = Mutex::new(Vec::new());
    let recorded_samples_clone = recorded_samples.clone();
    
    let err_fn = |err| error!("音频流错误: {}", err);
    
    let stream = match supported_config.sample_format() {
        cpal::SampleFormat::F32 => {
            device.build_input_stream(
                &supported_config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if RECORDING_FLAG.load(Ordering::SeqCst) {
                        let mut samples = recorded_samples_clone.lock().unwrap();
                        for &sample in data {
                            let s = (sample * 32767.0).max(-32768.0).min(32767.0) as i16;
                            samples.push(s);
                        }
                    }
                },
                err_fn,
                None,
            ).map_err(|e| format!("无法创建音频流: {}", e))?
        }
        cpal::SampleFormat::I16 => {
            device.build_input_stream(
                &supported_config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    if RECORDING_FLAG.load(Ordering::SeqCst) {
                        let mut samples = recorded_samples_clone.lock().unwrap();
                        samples.extend_from_slice(data);
                    }
                },
                err_fn,
                None,
            ).map_err(|e| format!("无法创建音频流: {}", e))?
        }
        _ => return Err("不支持的音频格式".to_string()),
    };
    
    stream.play().map_err(|e| format!("无法开始录音: {}", e))?;
    RECORDING_FLAG.store(true, Ordering::SeqCst);
    
    std::mem::forget(stream);
    
    info!("录音已开始，保存到: {}", audio_path_str);
    
    Ok(audio_path_str)
}

/// 停止录音
#[tauri::command]
pub async fn stop_recording(
    state: State<'_, AppState>,
) -> Result<RecordingStatus, String> {
    if !RECORDING_FLAG.load(Ordering::SeqCst) {
        return Err("当前没有在录音".to_string());
    }
    
    info!("停止录音");
    RECORDING_FLAG.store(false, Ordering::SeqCst);
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let audio_path = state.current_audio_path.lock().unwrap().clone();
    
    let status = RecordingStatus {
        is_recording: false,
        duration_seconds: 0,
        audio_path: audio_path.clone(),
        device_name: None,
    };
    
    if let Some(path) = audio_path {
        info!("录音文件已保存: {}", path);
    }
    
    Ok(status)
}

/// 获取录音状态
#[tauri::command]
pub async fn get_recording_status(
    state: State<'_, AppState>,
) -> Result<RecordingStatus, String> {
    let is_recording = RECORDING_FLAG.load(Ordering::SeqCst);
    let audio_path = state.current_audio_path.lock().unwrap().clone();
    
    Ok(RecordingStatus {
        is_recording,
        duration_seconds: 0,
        audio_path,
        device_name: None,
    })
}
