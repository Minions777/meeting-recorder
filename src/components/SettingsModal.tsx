import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '../hooks/useAppStore';

interface SettingsModalProps {
  onClose: () => void;
}

export function SettingsModal({ onClose }: SettingsModalProps) {
  const { config, setConfig } = useAppStore();
  
  const [activeTab, setActiveTab] = useState<'ai' | 'whisper' | 'general'>('ai');
  const [formData, setFormData] = useState({
    ai_provider: config?.ai_service?.provider || 'openai',
    ai_api_base: config?.ai_service?.api_base || 'https://api.openai.com/v1',
    ai_api_key: config?.ai_service?.api_key || '',
    ai_model: config?.ai_service?.model || 'gpt-4o-mini',
    ai_timeout: config?.ai_service?.timeout_seconds || 60,
    ai_use_proxy: config?.ai_service?.use_proxy || false,
    ai_proxy_url: config?.ai_service?.proxy_url || '',
    whisper_type: 'openai',
    whisper_api_base: 'https://api.openai.com/v1',
    whisper_api_key: '',
    whisper_model: 'whisper-1',
    theme: config?.general_config?.theme || 'light',
    auto_copy_summary: config?.general_config?.auto_copy_summary || false,
    notification_enabled: config?.general_config?.notification_enabled ?? true,
  });

  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null);

  const availableModels: Record<string, string[]> = {
    openai: ['gpt-4o', 'gpt-4o-mini', 'gpt-4-turbo', 'gpt-4', 'gpt-3.5-turbo'],
    anthropic: ['claude-3-5-sonnet-20241022', 'claude-3-5-haiku-20240307', 'claude-3-opus-20240229'],
    google: ['gemini-2.0-flash-exp', 'gemini-1.5-pro', 'gemini-1.5-flash'],
    qwen: ['qwen-plus', 'qwen-long', 'qwen-turbo', 'qwen-max'],
    zhipu: ['glm-4-plus', 'glm-4', 'glm-4-air', 'glm-3-turbo'],
  };

  const providerPresets: Record<string, { api_base: string }> = {
    openai: { api_base: 'https://api.openai.com/v1' },
    anthropic: { api_base: 'https://api.anthropic.com/v1' },
    google: { api_base: 'https://generativelanguage.googleapis.com/v1beta' },
    qwen: { api_base: 'https://dashscope.aliyuncs.com/compatible-mode/v1' },
    zhipu: { api_base: 'https://open.bigmodel.cn/api/paas/v4' },
  };

  useEffect(() => {
    if (providerPresets[formData.ai_provider]) {
      setFormData(prev => ({
        ...prev,
        ai_api_base: providerPresets[formData.ai_provider].api_base,
      }));
    }
  }, [formData.ai_provider]);

  const handleSave = async () => {
    try {
      const newConfig = {
        ai_service: {
          provider: formData.ai_provider,
          api_base: formData.ai_api_base,
          api_key: formData.ai_api_key,
          model: formData.ai_model,
          timeout_seconds: formData.ai_timeout,
          use_proxy: formData.ai_use_proxy,
          proxy_url: formData.ai_proxy_url,
        },
        whisper_config: {
          type: formData.whisper_type,
          api_base: formData.whisper_api_base,
          api_key: formData.whisper_api_key,
          model: formData.whisper_model,
        },
        recording_config: {
          default_sample_rate: 44100,
          default_channels: 1,
          auto_noise_reduction: true,
          enable_vad: true,
        },
        general_config: {
          language: 'zh-CN',
          theme: formData.theme,
          auto_copy_summary: formData.auto_copy_summary,
          notification_enabled: formData.notification_enabled,
        },
      };

      await invoke('save_config', { config: newConfig });
      setConfig(newConfig);
      alert('配置已保存');
      onClose();
    } catch (error) {
      console.error('保存配置失败:', error);
      alert(`保存失败: ${error}`);
    }
  };

  const handleTestConnection = async () => {
    setTesting(true);
    setTestResult(null);

    try {
      const tempConfig = {
        ai_service: {
          provider: formData.ai_provider,
          api_base: formData.ai_api_base,
          api_key: formData.ai_api_key,
          model: formData.ai_model,
          timeout_seconds: formData.ai_timeout,
          use_proxy: formData.ai_use_proxy,
          proxy_url: formData.ai_proxy_url,
        },
        whisper_config: {
          type: formData.whisper_type,
          api_base: formData.whisper_api_base,
          api_key: formData.whisper_api_key,
          model: formData.whisper_model,
        },
        recording_config: {
          default_sample_rate: 44100,
          default_channels: 1,
          auto_noise_reduction: true,
          enable_vad: true,
        },
        general_config: {
          language: 'zh-CN',
          theme: formData.theme,
          auto_copy_summary: false,
          notification_enabled: true,
        },
      };

      await invoke('save_config', { config: tempConfig });
      const result = await invoke<string>('test_connection');
      setTestResult({ success: true, message: result });
    } catch (error) {
      setTestResult({ success: false, message: String(error) });
    } finally {
      setTesting(false);
    }
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>⚙️ 设置</h2>
          <button className="btn-close" onClick={onClose}>×</button>
        </div>

        <div className="modal-tabs">
          <button 
            className={`tab-btn ${activeTab === 'ai' ? 'active' : ''}`}
            onClick={() => setActiveTab('ai')}
          >
            🤖 AI 服务
          </button>
          <button 
            className={`tab-btn ${activeTab === 'whisper' ? 'active' : ''}`}
            onClick={() => setActiveTab('whisper')}
          >
            🎤 转写服务
          </button>
          <button 
            className={`tab-btn ${activeTab === 'general' ? 'active' : ''}`}
            onClick={() => setActiveTab('general')}
          >
            ⚡ 通用设置
          </button>
        </div>

        <div className="modal-body">
          {activeTab === 'ai' && (
            <div className="settings-section">
              <div className="form-group">
                <label>AI 提供商</label>
                <select
                  value={formData.ai_provider}
                  onChange={(e) => setFormData({ ...formData, ai_provider: e.target.value })}
                >
                  <option value="openai">OpenAI</option>
                  <option value="anthropic">Anthropic (Claude)</option>
                  <option value="google">Google (Gemini)</option>
                  <option value="qwen">阿里通义千问</option>
                  <option value="zhipu">智谱 GLM</option>
                </select>
              </div>

              <div className="form-group">
                <label>API 地址</label>
                <input
                  type="text"
                  value={formData.ai_api_base}
                  onChange={(e) => setFormData({ ...formData, ai_api_base: e.target.value })}
                  placeholder="https://api.openai.com/v1"
                />
              </div>

              <div className="form-group">
                <label>API Key</label>
                <input
                  type="password"
                  value={formData.ai_api_key}
                  onChange={(e) => setFormData({ ...formData, ai_api_key: e.target.value })}
                  placeholder="sk-..."
                />
              </div>

              <div className="form-group">
                <label>模型</label>
                <select
                  value={formData.ai_model}
                  onChange={(e) => setFormData({ ...formData, ai_model: e.target.value })}
                >
                  {availableModels[formData.ai_provider]?.map((model) => (
                    <option key={model} value={model}>{model}</option>
                  ))}
                </select>
              </div>

              <div className="form-group">
                <label>超时时间 (秒)</label>
                <input
                  type="number"
                  value={formData.ai_timeout}
                  onChange={(e) => setFormData({ ...formData, ai_timeout: parseInt(e.target.value) })}
                  min={10}
                  max={300}
                />
              </div>

              <div className="form-group checkbox">
                <label>
                  <input
                    type="checkbox"
                    checked={formData.ai_use_proxy}
                    onChange={(e) => setFormData({ ...formData, ai_use_proxy: e.target.checked })}
                  />
                  使用代理
                </label>
              </div>

              {formData.ai_use_proxy && (
                <div className="form-group">
                  <label>代理地址</label>
                  <input
                    type="text"
                    value={formData.ai_proxy_url}
                    onChange={(e) => setFormData({ ...formData, ai_proxy_url: e.target.value })}
                    placeholder="http://127.0.0.1:7890"
                  />
                </div>
              )}

              <div className="test-connection">
                <button 
                  className="btn-secondary"
                  onClick={handleTestConnection}
                  disabled={testing || !formData.ai_api_key}
                >
                  {testing ? '测试中...' : '🔗 测试连接'}
                </button>
                {testResult && (
                  <span className={`test-result ${testResult.success ? 'success' : 'error'}`}>
                    {testResult.success ? '✓ ' : '✗ '}{testResult.message}
                  </span>
                )}
              </div>
            </div>
          )}

          {activeTab === 'whisper' && (
            <div className="settings-section">
              <div className="form-group">
                <label>转写服务类型</label>
                <select
                  value={formData.whisper_type}
                  onChange={(e) => setFormData({ ...formData, whisper_type: e.target.value })}
                >
                  <option value="openai">OpenAI Whisper API</option>
                  <option value="custom">自定义 API</option>
                </select>
              </div>

              <div className="form-group">
                <label>API 地址</label>
                <input
                  type="text"
                  value={formData.whisper_api_base}
                  onChange={(e) => setFormData({ ...formData, whisper_api_base: e.target.value })}
                  placeholder="https://api.openai.com/v1"
                />
              </div>

              <div className="form-group">
                <label>API Key</label>
                <input
                  type="password"
                  value={formData.whisper_api_key}
                  onChange={(e) => setFormData({ ...formData, whisper_api_key: e.target.value })}
                  placeholder="sk-..."
                />
              </div>

              <div className="form-group">
                <label>模型</label>
                <select
                  value={formData.whisper_model}
                  onChange={(e) => setFormData({ ...formData, whisper_model: e.target.value })}
                >
                  <option value="whisper-1">Whisper 1 (默认)</option>
                </select>
              </div>
            </div>
          )}

          {activeTab === 'general' && (
            <div className="settings-section">
              <div className="form-group">
                <label>主题</label>
                <select
                  value={formData.theme}
                  onChange={(e) => setFormData({ ...formData, theme: e.target.value })}
                >
                  <option value="light">浅色</option>
                  <option value="dark">深色</option>
                </select>
              </div>

              <div className="form-group checkbox">
                <label>
                  <input
                    type="checkbox"
                    checked={formData.auto_copy_summary}
                    onChange={(e) => setFormData({ ...formData, auto_copy_summary: e.target.checked })}
                  />
                  生成总结后自动复制到剪贴板
                </label>
              </div>

              <div className="form-group checkbox">
                <label>
                  <input
                    type="checkbox"
                    checked={formData.notification_enabled}
                    onChange={(e) => setFormData({ ...formData, notification_enabled: e.target.checked })}
                  />
                  启用桌面通知
                </label>
              </div>
            </div>
          )}
        </div>

        <div className="modal-footer">
          <button className="btn-secondary" onClick={onClose}>
            取消
          </button>
          <button className="btn-primary" onClick={handleSave}>
            💾 保存配置
          </button>
        </div>
      </div>
    </div>
  );
}
