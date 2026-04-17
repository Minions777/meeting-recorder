import React, { useRef, useEffect } from 'react';
import { useAppStore } from '../hooks/useAppStore';

export function TranscriptView() {
  const {
    transcript,
    transcriptSegments,
    isTranscribing,
    transcriptionProgress,
  } = useAppStore();

  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [transcript]);

  const formatTimestamp = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="transcript-view">
      <div className="panel-header">
        <h2 className="panel-title">转写文本</h2>
        {transcript && (
          <span className="char-count">{transcript.length} 字符</span>
        )}
      </div>

      <div className="transcript-content" ref={containerRef}>
        {isTranscribing ? (
          <div className="transcribing-indicator">
            <div className="loading-spinner" />
            <span>正在转写...</span>
            {transcriptionProgress > 0 && (
              <div className="progress-bar">
                <div 
                  className="progress-fill" 
                  style={{ width: `${transcriptionProgress}%` }}
                />
              </div>
            )}
          </div>
        ) : transcript ? (
          <div className="transcript-text">
            {transcriptSegments.length > 0 ? (
              transcriptSegments.map((segment, index) => (
                <div key={index} className="segment-item">
                  <span className="segment-timestamp">
                    {formatTimestamp(segment.start)}
                  </span>
                  <span className="segment-text">{segment.text}</span>
                </div>
              ))
            ) : (
              <div className="transcript-full">{transcript}</div>
            )}
          </div>
        ) : (
          <div className="empty-state">
            <div className="empty-icon">📝</div>
            <p>暂无转写内容</p>
            <p className="empty-hint">开始录音后将自动转写</p>
          </div>
        )}
      </div>

      {transcript && (
        <div className="transcript-actions">
          <button 
            className="btn-secondary"
            onClick={() => navigator.clipboard.writeText(transcript)}
          >
            📋 复制全文
          </button>
        </div>
      )}
    </div>
  );
}
