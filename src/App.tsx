import React, { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { RecordingPanel } from './components/RecordingPanel';
import { TranscriptView } from './components/TranscriptView';
import { SummaryPanel } from './components/SummaryPanel';
import { SettingsModal } from './components/SettingsModal';
import { useAppStore } from './hooks/useAppStore';
import { AudioDevice, AppConfig } from './types';
import './styles/global.css';

function App() {
  const {
    isRecording,
    transcript,
    summary,
    showSettings,
    setShowSettings,
    setAudioDevices,
    setConfig,
  } = useAppStore();

  useEffect(() => {
    const init = async () => {
      try {
        const devices = await invoke<AudioDevice[]>('get_audio_devices');
        setAudioDevices(devices);
        const config = await invoke<AppConfig>('load_config');
        setConfig(config);
      } catch (error) {
        console.error('初始化失败:', error);
      }
    };
    init();
  }, []);

  return (
    <div className="app-container">
      <header className="app-header">
        <h1 className="app-title">📝 会议记录助手</h1>
        <div className="header-actions">
          <button 
            className="btn-icon"
            onClick={() => setShowSettings(true)}
            title="设置"
          >
            ⚙️
          </button>
        </div>
      </header>

      <main className="app-main">
        <aside className="panel-left">
          <RecordingPanel />
        </aside>

        <section className="panel-center">
          <TranscriptView />
        </section>

        <aside className="panel-right">
          <SummaryPanel />
        </aside>
      </main>

      <footer className="app-footer">
        <div className="status-indicator">
          {isRecording ? (
            <span className="status-recording">🔴 录音中</span>
          ) : (
            <span className="status-idle">⚪ 待机</span>
          )}
        </div>
        <div className="status-info">
          {transcript && <span>已转写 {transcript.length} 字符</span>}
          {summary && <span> | 已生成总结</span>}
        </div>
      </footer>

      {showSettings && <SettingsModal onClose={() => setShowSettings(false)} />}
    </div>
  );
}

export default App;
