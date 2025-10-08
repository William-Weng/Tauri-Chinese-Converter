use ferrous_opencc::{OpenCC};

/// 繁簡轉換
/// @param text - 要轉換的文字
/// @param config - 轉換配置 ("s2t" 或 "t2s")
/// @returns 轉換後的文字
#[tauri::command]
fn convert_text(text: &str, config: &str) -> String {
    
    let mut _config = ferrous_opencc::config::BuiltinConfig::S2t;

    if config == "t2s" { _config = ferrous_opencc::config::BuiltinConfig::T2s; }

    let opencc = OpenCC::from_config(_config).unwrap();
    let converted = opencc.convert(text);

    return converted;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
