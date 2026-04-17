import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '../hooks/useAppStore';

export function SummaryPanel() {
  const {
    transcript,
    summary,
    setSummary,
    isSummarizing,
    setIsSummarizing,
  } = useAppStore();

  const [error, setError] = useState<string | null>(null);

  const handleSummarize = async () => {
    if (!transcript) {
      setError('请先转写会议内容');
      return;
    }

    setIsSummarizing(true);
    setError(null);

    try {
      const result = await invoke<any>('summarize_text', {
        transcript,
      });
      setSummary(result);
    } catch (err) {
      console.error('总结失败:', err);
      setError(`总结失败: ${err}`);
    } finally {
      setIsSummarizing(false);
    }
  };

  const copySummary = async (format: 'markdown' | 'text') => {
    if (!summary) return;

    let content = '';
    
    if (format === 'markdown') {
      const date = summary.meeting_info.date || new Date().toLocaleDateString('zh-CN');
      content = `# 会议纪要 - ${date}

## 会议概要
${summary.summary}

## 关键讨论点
${summary.key_points.map((point, i) => `${i + 1}. ${point}`).join('\n')}

## 决策事项
${summary.decisions.length > 0 
  ? summary.decisions.map(d => `- ${d}`).join('\n') 
  : '无'}

## 待办任务
${summary.action_items.length > 0 
  ? summary.action_items.map(item => 
      `- [ ] ${item.task}${item.assignee ? ` - ${item.assignee}` : ''}${item.deadline ? ` (${item.deadline})` : ''}`
    ).join('\n')
  : '无'}

## 关键词
${summary.keywords.map(k => `#${k}`).join(' ')}`;
    } else {
      content = JSON.stringify(summary, null, 2);
    }

    try {
      await invoke('copy_to_clipboard', { text: content });
      alert('已复制到剪贴板');
    } catch (err) {
      console.error('复制失败:', err);
      alert('复制失败');
    }
  };

  return (
    <div className="summary-panel">
      <div className="panel-header">
        <h2 className="panel-title">会议总结</h2>
      </div>

      <div className="summary-actions">
        {!summary ? (
          <button
            className="btn-primary summarize-btn"
            onClick={handleSummarize}
            disabled={!transcript || isSummarizing}
          >
            {isSummarizing ? (
              <>
                <span className="loading-spinner small" />
                生成中...
              </>
            ) : (
              <>🧠 生成总结</>
            )}
          </button>
        ) : (
          <div className="copy-buttons">
            <button className="btn-secondary" onClick={() => copySummary('markdown')}>
              📋 复制 Markdown
            </button>
            <button className="btn-secondary" onClick={() => copySummary('text')}>
              📄 复制 JSON
            </button>
          </div>
        )}
      </div>

      {error && (
        <div className="error-message">{error}</div>
      )}

      <div className="summary-content">
        {isSummarizing ? (
          <div className="loading-state">
            <div className="loading-spinner large" />
            <p>正在分析会议内容...</p>
          </div>
        ) : summary ? (
          <div className="summary-cards">
            <div className="summary-card">
              <h3 className="card-title">📋 会议概要</h3>
              <p className="summary-text">{summary.summary}</p>
            </div>

            {summary.key_points.length > 0 && (
              <div className="summary-card">
                <h3 className="card-title">💡 关键讨论点</h3>
                <ul className="key-points-list">
                  {summary.key_points.map((point, index) => (
                    <li key={index}>{point}</li>
                  ))}
                </ul>
              </div>
            )}

            {summary.decisions.length > 0 && (
              <div className="summary-card decisions">
                <h3 className="card-title">✅ 决策事项</h3>
                <ul className="decisions-list">
                  {summary.decisions.map((decision, index) => (
                    <li key={index}>{decision}</li>
                  ))}
                </ul>
              </div>
            )}

            {summary.action_items.length > 0 && (
              <div className="summary-card tasks">
                <h3 className="card-title">📌 待办任务</h3>
                <ul className="tasks-list">
                  {summary.action_items.map((item, index) => (
                    <li key={index} className="task-item">
                      <input type="checkbox" />
                      <span className="task-text">{item.task}</span>
                      {item.assignee && (
                        <span className="task-assignee">@{item.assignee}</span>
                      )}
                      {item.deadline && (
                        <span className="task-deadline">📅 {item.deadline}</span>
                      )}
                    </li>
                  ))}
                </ul>
              </div>
            )}

            {summary.keywords.length > 0 && (
              <div className="summary-card keywords">
                <h3 className="card-title">🏷️ 关键词</h3>
                <div className="keywords-list">
                  {summary.keywords.map((keyword, index) => (
                    <span key={index} className="keyword-tag">
                      #{keyword}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        ) : (
          <div className="empty-state">
            <div className="empty-icon">🧠</div>
            <p>暂无总结内容</p>
            <p className="empty-hint">转写完成后点击生成总结</p>
          </div>
        )}
      </div>
    </div>
  );
}
