use keyring::Entry;

// 服务名称与键名，用于在操作系统凭据库中标识你的应用
const SERVICE_NAME: &str = "magno-translator";
const KEY_NAME: &str = "google_api_key";

// 1. 供前端调用的保存 Key 命令（加密存入系统安全凭据）
#[tauri::command]
pub fn save_api_key(api_key: String) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEY_NAME)
        .map_err(|e| format!("初始化凭据库失败: {}", e))?;

    entry.set_password(&api_key)
        .map_err(|e| format!("保存 API Key 失败: {}", e))?;

    Ok(())
}

// 2. 供后端或前端读取 Key 的函数（从系统安全凭据库安全解密读取）
pub fn get_saved_api_key(_app_handle: &tauri::AppHandle) -> String {
    match Entry::new(SERVICE_NAME, KEY_NAME) {
        Ok(entry) => {
            match entry.get_password() {
                Ok(key) => key,
                Err(_) => String::new(), // 如果没有存过或读取失败，返回空字符串
            }
        }
        Err(_) => String::new(),
    }
}

// 3. 供前端调用的加载 Key 命令
#[tauri::command]
pub fn load_api_key(app_handle: tauri::AppHandle) -> String {
    get_saved_api_key(&app_handle)
}