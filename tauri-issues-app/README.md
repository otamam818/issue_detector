# Tauri Database issue
This is a template developed with Tauri, Solid and Typescript in Vite.

It demonstrates the error that occurs when adding a database crate to the app.

## Replication
On Windows, run the following commands:
```powershell
# Install the latest version of `create-tauri-app` and create the tauri-issues-app folder
cargo install create-tauri-app
cargo create-tauri-app --beta

# Settings chosen:
# Language: TypeScript
# Framework: SolidJS
# Android support: yes
# Project name: tauri-issues-app

# Open the directory and run the init instructions
cd tauri-issues-app
npm install
npm run tauri android init
```

Add the following to the following files:
- `src-tauri/Cargo.toml`:
```toml
rusqlite = { version = "0.29.0", features = ["bundled"] }
```

- `src-tauri/src/lib.rs`:
```rust
#[tauri::command]
fn greet(name: &str, app: tauri::AppHandle) -> String {
    let app_data_dir = app.path().app_data_dir().unwrap().to_str().unwrap().to_string();
    let database_path = format!("{}/database.db", app_data_dir);
    let _conn = rusqlite::Connection::open(database_path).unwrap();

    format!("Hello, {}! You've been greeted from Rust!\n\n{}", name)
}
```

```powershell
# Run the app in Android
# NOTE: Make sure an android emulator is installed. Easiest way to do this is via
#       using Android studio
npm run tauri android dev
```

## Solution
A solution will be provided once figured out or fixed
