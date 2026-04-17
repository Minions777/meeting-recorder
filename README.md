# 会议记录助手

📝 轻量化、高兼容性、强隐私保护的本地会议记录工具

## 功能特点

- 🎤 **语音录制**：支持麦克风和系统音频录制
- 🔊 **Whisper 转写**：兼容 OpenAI Whisper API，高准确率转写
- 🤖 **AI 总结**：支持多厂商 AI 服务，智能生成会议纪要
- 📋 **一键复制**：Markdown 格式输出，直接粘贴使用
- 🔒 **隐私保护**：本地处理，数据不经过第三方服务器
- 🌐 **多厂商支持**：OpenAI、Claude、Gemini、通义千问、智谱清言

## 技术栈

- **前端**：React + TypeScript + Vite
- **后端**：Rust + Tauri 2.0
- **音频**：CPAL (Rust 音频处理)
- **AI 服务**：OpenAI 兼容协议

## 快速开始

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建发布
npm run tauri build
```

## License

MIT License
