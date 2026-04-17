use serde::{Deserialize, Serialize};
use log::{info, error};
use reqwest::Client;
use crate::config::AIServiceConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryResult {
    pub summary: String,
    pub key_points: Vec<String>,
    pub decisions: Vec<String>,
    pub action_items: Vec<ActionItem>,
    pub keywords: Vec<String>,
    pub meeting_info: MeetingInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub task: String,
    pub assignee: Option<String>,
    pub deadline: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingInfo {
    pub date: String,
    pub duration_minutes: Option<u32>,
    pub topic: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIChatMessage {
    pub role: String,
    pub content: String,
}

fn get_summary_system_prompt() -> String {
    r#"你是一个专业的会议记录助手，负责从会议转写文本中提取关键信息并生成结构化的会议纪要。

请分析提供的会议转写文本，并按照以下JSON格式返回结果：

{
    "summary": "会议概要（100-200字）",
    "key_points": ["关键讨论点1", "关键讨论点2", ...],
    "decisions": ["决策事项1", "决策事项2", ...],
    "action_items": [
        {"task": "任务描述", "assignee": "负责人（如有）", "deadline": "截止时间（如有）"},
        ...
    ],
    "keywords": ["关键词1", "关键词2", ...],
    "meeting_info": {
        "date": "会议日期",
        "duration_minutes": 会议时长（分钟）,
        "topic": "会议主题（如能识别）"
    }
}

注意事项：
1. 只返回JSON格式，不要包含任何其他文字
2. 如果某项没有信息，使用空数组或null
3. action_items中的assignee和deadline如果无法确定，设置为null
4. keywords提取5-8个核心关键词
5. summary要用中文撰写"#.to_string()
}

async fn call_chat_api(
    endpoint: &str,
    api_key: &str,
    model: &str,
    messages: Vec<AIChatMessage>,
    max_tokens: Option<u32>,
) -> Result<String, String> {
    let client = Client::new();
    
    let mut request_body = serde_json::json!({
        "model": model,
        "messages": messages.iter().map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content
            })
        }).collect::<Vec<_>>(),
        "temperature": 0.3,
    });
    
    if let Some(tokens) = max_tokens {
        request_body["max_tokens"] = serde_json::json!(tokens);
    }
    
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("API 请求失败: {}", e))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        error!("AI API 返回错误: {}", error_text);
        return Err(format!("AI 服务返回错误: {}", error_text));
    }
    
    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;
    
    let content = result["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("无法从响应中提取内容")?;
    
    Ok(content.to_string())
}

#[tauri::command]
pub async fn summarize_text(
    transcript: String,
    state: tauri::State<'_, crate::AppState>,
) -> Result<SummaryResult, String> {
    info!("开始生成会议总结");
    
    let config = state.config.lock().unwrap().clone();
    
    let ai_config = match &config.ai_service {
        Some(cfg) => cfg,
        None => return Err("请先配置 AI 服务".to_string()),
    };
    
    let messages = vec![
        AIChatMessage {
            role: "system".to_string(),
            content: get_summary_system_prompt(),
        },
        AIChatMessage {
            role: "user".to_string(),
            content: format!("请分析以下会议转写文本并生成结构化纪要：\n\n{}", transcript),
        },
    ];
    
    let endpoint = format!("{}/chat/completions", ai_config.api_base.trim_end_matches('/'));
    
    info!("调用 AI 服务: {}", endpoint);
    
    let response_content = match ai_config.provider.as_str() {
        "anthropic" => {
            call_anthropic_api(&endpoint, &ai_config.api_key, &ai_config.model, messages).await?
        }
        _ => {
            call_chat_api(
                &endpoint,
                &ai_config.api_key,
                &ai_config.model,
                messages,
                Some(4000),
            ).await?
        }
    };
    
    info!("AI 返回原始内容，尝试解析...");
    
    let result_json: serde_json::Value = serde_json::from_str(&response_content)
        .or_else(|_| {
            let json_start = response_content.find("{").unwrap_or(0);
            let json_end = response_content.rfind("}").map(|i| i + 1).unwrap_or(response_content.len());
            serde_json::from_str(&response_content[json_start..json_end])
        })
        .map_err(|e| format!("无法解析 AI 返回的 JSON: {}，原始内容: {}", e, response_content))?;
    
    let summary = result_json["summary"]
        .as_str()
        .unwrap_or("无法生成概要")
        .to_string();
    
    let key_points = result_json["key_points"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();
    
    let decisions = result_json["decisions"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();
    
    let mut action_items = Vec::new();
    if let Some(arr) = result_json["action_items"].as_array() {
        for item in arr {
            action_items.push(ActionItem {
                task: item["task"].as_str().unwrap_or("").to_string(),
                assignee: item["assignee"].as_str().map(String::from),
                deadline: item["deadline"].as_str().map(String::from),
            });
        }
    }
    
    let keywords = result_json["keywords"]
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();
    
    let meeting_info = MeetingInfo {
        date: result_json["meeting_info"]["date"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        duration_minutes: result_json["meeting_info"]["duration_minutes"].as_u64().map(|v| v as u32),
        topic: result_json["meeting_info"]["topic"].as_str().map(String::from),
    };
    
    info!("总结生成成功");
    
    Ok(SummaryResult {
        summary,
        key_points,
        decisions,
        action_items,
        keywords,
        meeting_info,
    })
}

async fn call_anthropic_api(
    endpoint: &str,
    api_key: &str,
    model: &str,
    messages: Vec<AIChatMessage>,
) -> Result<String, String> {
    let client = Client::new();
    
    let anthropic_messages: Vec<serde_json::Value> = messages
        .iter()
        .filter(|m| m.role != "system")
        .map(|m| {
            serde_json::json!({
                "role": if m.role == "assistant" { "assistant" } else { "user" },
                "content": m.content
            })
        })
        .collect();
    
    let system_prompt = messages
        .iter()
        .find(|m| m.role == "system")
        .map(|m| m.content.clone())
        .unwrap_or_default();
    
    let request_body = serde_json::json!({
        "model": model,
        "max_tokens": 4000,
        "system": system_prompt,
        "messages": anthropic_messages,
    });
    
    let response = client
        .post(endpoint)
        .header("x-api-key", api_key)
        .header("Content-Type", "application/json")
        .header("anthropic-version", "2023-06-01")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Anthropic API 请求失败: {}", e))?;
    
    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        error!("Anthropic API 返回错误: {}", error_text);
        return Err(format!("Anthropic API 返回错误: {}", error_text));
    }
    
    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;
    
    result["content"][0]["text"]
        .as_str()
        .map(String::from)
        .ok_or("无法从 Anthropic 响应中提取内容".to_string())
}

#[tauri::command]
pub async fn test_connection(
    provider: String,
    api_base: String,
    api_key: String,
    model: String,
    _timeout_seconds: u64,
    use_proxy: bool,
    proxy_url: String,
) -> Result<String, String> {
    info!("测试 AI 服务连接...");

    if api_key.is_empty() {
        return Err("请先输入 API Key".to_string());
    }

    let messages = vec![
        AIChatMessage {
            role: "user".to_string(),
            content: "你好，这是一个连接测试。请回复 '连接成功'。".to_string(),
        },
    ];

    let endpoint = format!("/chat/completions", api_base.trim_end_matches('/'));

    // Handle proxy if needed
    let client = if use_proxy && !proxy_url.is_empty() {
        let proxy = reqwest::Proxy::https(&proxy_url)
            .map_err(|e| format!("代理配置错误: {}", e))?;
        Client::builder().proxy(proxy).build()
    } else {
        Client::builder().build()
    }
    .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let request_body = serde_json::json!({
        "model": model,
        "messages": messages.iter().map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content
            })
        }).collect::<Vec<_>>(),
        "temperature": 0.3,
        "max_tokens": 100,
    });

    let full_url = if api_base.starts_with("http") {
        format!("{}{}", api_base.trim_end_matches('/'), endpoint)
    } else {
        format!("https://{}{}", api_base.trim_end_matches('/'), endpoint)
    };

    let response = client
        .post(&full_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("API 请求失败: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        error!("AI API 返回错误: {}", error_text);
        return Err(format!("AI 服务返回错误: {}", error_text));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    let content = result["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("无法从响应中提取内容")?;

    info!("连接测试成功: {}", content);

    Ok(content.to_string())
}

#[tauri::command]
pub fn get_available_models(provider: String) -> Vec<String> {
    match provider.as_str() {
        "openai" => vec![
            "gpt-4o".to_string(),
            "gpt-4o-mini".to_string(),
            "gpt-4-turbo".to_string(),
            "gpt-4".to_string(),
            "gpt-3.5-turbo".to_string(),
        ],
        "anthropic" => vec![
            "claude-3-5-sonnet-20241022".to_string(),
            "claude-3-5-haiku-20240307".to_string(),
            "claude-3-opus-20240229".to_string(),
        ],
        "google" => vec![
            "gemini-2.0-flash-exp".to_string(),
            "gemini-1.5-pro".to_string(),
            "gemini-1.5-flash".to_string(),
        ],
        "qwen" => vec![
            "qwen-plus".to_string(),
            "qwen-long".to_string(),
            "qwen-turbo".to_string(),
        ],
        "zhipu" => vec![
            "glm-4-plus".to_string(),
            "glm-4".to_string(),
            "glm-4-air".to_string(),
        ],
        _ => vec![],
    }
}
