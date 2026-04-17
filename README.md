# 会议记录助手

📝 轻量化、高兼容性、强隐私保护的本地会议记录工具

## 功能特点

- 🎤 **语音录制**：支持麦克风和系统音频录制
- 🔊 **Whisper 转写**：兼容 OpenAI Whisper API，高准确率转写
- 🤖 **AI 总结**：支持多厂商 AI 服务，智能生成会议纪要
- 📋 **一键复制**：Markdown 格式输出，直接粘贴使用
- 🔒 **隐私保护**：本地处理，数据不经过第三方服务器
- 🌐 **多厂商支持**：OpenAI、Claude、Gemini、通义千问、智谱清言

## 支持的 AI 服务商

| 提供商 | 模型示例 |
|--------|----------|
| OpenAI | GPT-4o, GPT-4o-mini, GPT-4-turbo |
| Anthropic | Claude-3.5-Sonnet, Claude-3-Haiku |
| Google | Gemini-2.0, Gemini-1.5-Pro |
| 阿里通义 | qwen-plus, qwen-long |
| 智谱清言 | GLM-4-plus, GLM-4 |

## 技术栈

- **前端**：React + TypeScript + Vite
- **后端**：Rust + Tauri 2.0
- **音频**：CPAL (Rust 音频处理)
- **AI 服务**：OpenAI 兼容协议

## 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Windows 10/11

### 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# 安装 Rust 依赖
cd src-tauri
cargo build
```

### 开发模式

```bash
npm run tauri dev
```

### 构建发布

```bash
npm run tauri build
```

## 使用流程

1. **配置 AI 服务**
   - 点击右上角 ⚙️ 设置按钮
   - 选择 AI 提供商并输入 API Key
   - 配置 Whisper 转写服务

2. **开始录音**
   - 选择音频输入设备
   - 点击「开始录音」按钮

3. **自动处理**
   - 停止录音后自动转写
   - 转写完成后生成总结

4. **复制使用**
   - 点击「复制 Markdown」一键复制
   - 直接粘贴到文档工具中

## 目录结构

```
meeting-recorder/
├── src/                    # React 前端源码
│   ├── components/         # UI 组件
│   │   ├── RecordingPanel.tsx  # 录音控制面板
│   │   ├── TranscriptView.tsx # 转写文本显示
│   │   ├── SummaryPanel.tsx   # 总结结果面板
│   │   └── SettingsModal.tsx  # 设置弹窗
│   ├── hooks/             # 自定义 Hooks
│   │   └── useAppStore.ts # Zustand 状态管理
│   ├── types/             # TypeScript 类型定义
│   └── styles/            # 样式文件
├── src-tauri/              # Rust 后端源码
│   └── src/
│       ├── main.rs        # 主入口
│       ├── lib.rs         # 库入口
│       ├── audio.rs       # 音频录制模块
│       ├── transcription.rs # 转写服务模块
│       ├── ai_service.rs   # AI 服务适配器
│       ├── config.rs      # 配置管理
│       └── utils.rs       # 工具函数
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md
```

## 配置说明

### AI 服务配置

```json
{
  "provider": "openai",
  "api_base": "https://api.openai.com/v1",
  "api_key": "your-api-key",
  "model": "gpt-4o-mini",
  "timeout_seconds": 60
}
```

### Whisper 配置

```json
{
  "type": "openai",
  "api_base": "https://api.openai.com/v1",
  "api_key": "your-api-key",
  "model": "whisper-1"
}
```

## 成本说明

- **Whisper API**：$0.006/分钟（约 1 小时会议 $0.36）
- **GPT-4o-mini**：$0.15/百万 tokens（总结约 500 tokens = $0.000075）
- **单次会议成本**：不到 1 分钱

## 隐私声明

- 音频文件仅用于转写，不保存到任何服务器
- 所有处理在本地完成
- AI 服务调用使用加密传输

## License

MIT License
