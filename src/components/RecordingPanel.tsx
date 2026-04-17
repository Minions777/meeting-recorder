import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '../hooks/useAppStore';

export function RecordingPanel() {
  const {
    isRecording,
    setIsRecording,
    recordingDuration,
    setRecordingDuration,
    audioDevices,
    selectedDevice,
    setSelectedDevice,
    setAudioPath,
    setTranscript,
    setIsTranscribing,
  } = useAppStore();

  const [audioLevel, setAudioLevel] = useState(0);
  const timerRef = useRef<number | null>(null);
  const startTimeRef = useRef<number>(0);

  useEffect(() => {
    if (isRecording) {
      startTimeRef.current = Date.now();
      timerRef.current = window.setInterval(() => {
        const elapsed = Math.floor((Date.now() - startTimeRef.current) / 1000);
        setRecordingDuration(elapsed);
      }, 100);
    } else {
      if (timerRef.current) {
        clearInterval(timerRef.current);
        timerRef.current = null;
      }
    }

    return () => {
      if (timerRef.current) {
        clearInterval(timerRef.current);
      }
    };
  }, [isRecording]);

  const handleStartRecording = async () => {
    try {
      const audioPath = await invoke<string>('start_recording', {
        config: {
          sample_rate: 44100,
          channels: 1,
          device_id: selectedDevice,
          recording_mode: 'Microphone',
        },
      });
      
      setAudioPath(audioPath);
      setIsRecording(true);
      setRecordingDuration(0);
    } catch (error) {
      console.error('开始录音失败:', error);
      alert(`开始录音失败: ${error}`);
    }
  };

  const handleStopRecording = async () => {
    try {
      const status = await invoke<any>('stop_recording');
      setIsRecording(false);
      setAudioPath(status.audio_path);
      
      if (status.audio_path) {
        handleTranscribe(status.audio_path);
      }
    } catch (error) {
      console.error('停止录音失败:', error);
      setIsRecording(false);
    }
  };

  const handleTranscribe = async (audioPath: string) => {
    if (!audioPath) return;
    
    setIsTranscribing(true);
    setTranscript('');
    
    try {
      const result = await invoke<any>('transcribe_audio_file', {
        audioPath,
      });
      
      setTranscript(result.text);
    } catch (error) {
      console.error('转写失败:', error);
      alert(`转写失败: ${error}`);
    } finally {
      setIsTranscribing(false);
    }
  };

  const formatDuration = (seconds: number): string => {
    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    if (hrs > 0) {
      return `${hrs.toString().padStart(2, '0')}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
    }
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="recording-panel">
      <h2 className="panel-title">录音控制</h2>
      
      <div className="device-selector">
        <label htmlFor="audio-device">输入设备</label>
        <select
          id="audio-device"
          value={selectedDevice || ''}
          onChange={(e) => setSelectedDevice(e.target.value)}
          disabled={isRecording}
        >
          <option value="">默认设备</option>
          {audioDevices.map((device) => (
            <option key={device.id} value={device.id}>
              {device.name} {device.is_default && '(默认)'}
            </option>
          ))}
        </select>
      </div>

      <div className="waveform-container">
        <div className="waveform">
          {Array.from({ length: 20 }).map((_, i) => (
            <div
              key={i}
              className={`waveform-bar ${isRecording ? 'active' : ''}`}
              style={{
                height: isRecording 
                  ? `${Math.random() * 60 + 20}%` 
                  : '10%',
                animationDelay: `${i * 0.05}s`,
              }}
            />
          ))}
        </div>
      </div>

      <div className="recording-duration">
        <span className="duration-time">{formatDuration(recordingDuration)}</span>
      </div>

      <button
        className={`record-button ${isRecording ? 'recording' : ''}`}
        onClick={isRecording ? handleStopRecording : handleStartRecording}
      >
        {isRecording ? (
          <>
            <span className="stop-icon">⏹</span>
            停止录音
          </>
        ) : (
          <>
            <span className="record-icon">🎙</span>
            开始录音
          </>
        )}
      </button>

      <div className="recording-hint">
        {isRecording ? (
          <p className="hint-recording">正在录音，点击停止后将自动转写</p>
        ) : (
          <p className="hint-idle">选择设备后点击开始录音</p>
        )}
      </div>
    </div>
  );
}
