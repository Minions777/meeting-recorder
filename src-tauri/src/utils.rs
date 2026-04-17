use tauri::Manager;
use tauri_plugin_clipboard_manager::ClipboardExt;
use log::info;
use std::fs;

#[tauri::command]
pub fn copy_to_clipboard(
    text: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    info!("复制到剪贴板，长度: {} 字符", text.len());
    
    app.clipboard()
        .write_text(&text)
        .map_err(|e| format!("无法复制到剪贴板: {}", e))?;
    
    info!("已成功复制到剪贴板");
    Ok(())
}

#[tauri::command]
pub async fn export_to_file(
    content: String,
    file_path: String,
    format: String,
) -> Result<String, String> {
    info!("导出文件: {}，格式: {}", file_path, format);
    
    let final_content = match format.as_str() {
        "markdown" => content,
        "json" => {
            serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&content)
                .unwrap_or(serde_json::json!({ "content": content })))
                .unwrap_or(content)
        }
        "text" | _ => content,
    };
    
    if let Some(parent) = std::path::Path::new(&file_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("无法创建目录: {}", e))?;
    }
    
    fs::write(&file_path, final_content)
        .map_err(|e| format!("无法写入文件: {}", e))?;
    
    info!("文件已导出: {}", file_path);
    
    Ok(file_path)
}

#[tauri::command]
pub fn get_app_info() -> serde_json::Value {
    serde_json::json!({
        "name": "会议记录助手",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "轻量化、高兼容性、强隐私保护的本地会议记录工具",
        "author": env!("CARGO_PKG_AUTHORS"),
    })
}
