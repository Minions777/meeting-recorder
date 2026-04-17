// 音频设备
export interface AudioDevice {
  name: string;
  id: string;
  is_default: boolean;
  is_input: boolean;
}

// 录音配置
export interface RecordingConfig {
  sample_rate: number;
  channels: number;
  device_id?: string;
  recording_mode: 'Microphone' | 'SystemAudio';
}

// 录音状态
export interface RecordingStatus {
  is_recording: boolean;
  duration_seconds: number;
  audio_path?: string;
  device_name?: string;
}

// 转写结果
export interface TranscriptionSegment {
  start: number;
  end: number;
  text: string;
}

export interface TranscriptionResult {
  text: string;
  language?: string;
  segments: TranscriptionSegment[];
  duration: number;
}

// AI 服务配置
export interface AIServiceConfig {
  provider: 'openai' | 'anthropic' | 'google' | 'qwen' | 'zhipu';
  api_base: string;
  api_key: string;
  model: string;
  timeout_seconds: number;
  use_proxy: boolean;
  proxy_url?: string;
}

// Whisper 配置
export interface WhisperConfig {
  type: 'openai' | 'custom';
  api_base: string;
  api_key: string;
  model: string;
}

// 待办事项
export interface ActionItem {
  task: string;
  assignee?: string;
  deadline?: string;
}

// 会议信息
export interface MeetingInfo {
  date: string;
  duration_minutes?: number;
  topic?: string;
}

// 总结结果
export interface SummaryResult {
  summary: string;
  key_points: string[];
  decisions: string[];
  action_items: ActionItem[];
  keywords: string[];
  meeting_info: MeetingInfo;
}

// 应用配置
export interface AppConfig {
  ai_service?: AIServiceConfig;
  whisper_config: WhisperConfig;
  recording_config: RecordingSettings;
  general_config: GeneralSettings;
}

export interface RecordingSettings {
  default_sample_rate: number;
  default_channels: number;
  auto_noise_reduction: boolean;
  enable_vad: boolean;
}

export interface GeneralSettings {
  language: string;
  theme: 'light' | 'dark';
  auto_copy_summary: boolean;
  notification_enabled: boolean;
}

// 应用状态
export interface AppState {
  isRecording: boolean;
  recordingDuration: number;
  audioDevices: AudioDevice[];
  selectedDevice: string | null;
  audioPath: string | null;
  transcript: string;
  transcriptSegments: TranscriptionSegment[];
  isTranscribing: boolean;
  transcriptionProgress: number;
  summary: SummaryResult | null;
  isSummarizing: boolean;
  config: AppConfig | null;
  showSettings: boolean;
  notification: string | null;
}
