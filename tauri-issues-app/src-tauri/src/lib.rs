use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str, app: tauri::AppHandle) -> String {
    let app_data_dir = app.path().app_data_dir().unwrap().to_str().unwrap().to_string();
    let mut extra_text = format!("app_data_dir path: {}", app_data_dir);

    let database_path = format!("{}/database.db", app_data_dir);
    extra_text.push_str(&format!("\nDatabase path: {}", &database_path));

    match std::fs::metadata(&database_path) {
      Ok(_value) => {
        extra_text.push_str("\n Database does exist, creating new one");
      },
      Err(_kind) => {
        extra_text.push_str("\n Database doesn't exist, connecting to it");
      }
    };

    /*
    let _conn = sqlx::sqlite::SqlitePoolOptions::new()
      .connect(&database_path);
     */
    let conn = rusqlite::Connection::open(database_path);
    match conn {
      Ok(_conn) => {
        extra_text.push_str(" Successfully connected to database");
      },
      Err(kind) => {
        extra_text.push_str(&format!("Couldn't connect with error: {kind:?}"));
      }
    };

    format!(
      "Hello, {}! You've been greeted from Rust!\n\n{}",
      name,
      extra_text
    )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
