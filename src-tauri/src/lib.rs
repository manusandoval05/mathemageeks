// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn eval_math_expression(expression: &str) -> String {
    format!("Expresión: {}", expression)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![eval_math_expression])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
