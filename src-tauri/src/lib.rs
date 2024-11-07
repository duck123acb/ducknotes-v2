// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn style(content: String, selection_start: i32, selection_end: i32, style_type: String) -> String {
  format!("{}", content)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![style])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
