# Tauri Database issue
This is a template developed with Tauri, Solid and Typescript in Vite.

It demonstrates the error that occurs when adding a database crate to the app.

## System
> **Operating System:** Windows 10 (version 22H2)  
> **Android Emulator:** Pixel 7 Pro API 34  
> **JDK Version:** openjdk-17.0.1  
> **Tauri Version:** 2.0.0-beta  
> **Noted from:** 15th February 2024  

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
- `src-tauri/Cargo.toml` (in the `[dependencies]` section):
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

Make sure an android emulator is installed. Easiest way to do this is via using Android studio. Then run the app in Android:
```powershell
npm run tauri android dev
```

## Issue
The app crashes. When opening it again, it shows the following message

```C++
--------- beginning of main
02-17 11:59:37.989  3114  3114 I auri_issues_app: Late-enabling -Xcheck:jni
02-17 11:59:38.009  3114  3114 I auri_issues_app: Using CollectorTypeCC GC.
02-17 11:59:38.010  3114  3114 W auri_issues_app: Unexpected CPU variant for x86: x86_64.
02-17 11:59:38.010  3114  3114 W auri_issues_app: Known variants: atom, sandybridge, silvermont, goldmont, goldmont-plus, tremont, kabylake, default
02-17 11:59:38.085  3114  3114 W ziparchive: Unable to open '/data/app/~~oWhhee07OzLL55nrtCgF1A==/com.tauri.tauri_issues_app-6AshWwk-VAC3roCMDl_dDA==/base.dm': No such file or directory
02-17 11:59:38.085  3114  3114 W ziparchive: Unable to open '/data/app/~~oWhhee07OzLL55nrtCgF1A==/com.tauri.tauri_issues_app-6AshWwk-VAC3roCMDl_dDA==/base.dm': No such file or directory
--------- beginning of crash
02-17 11:59:38.324  3114  3114 E AndroidRuntime: FATAL EXCEPTION: main
02-17 11:59:38.324  3114  3114 E AndroidRuntime: Process: com.tauri.tauri_issues_app, PID: 3114
02-17 11:59:38.324  3114  3114 E AndroidRuntime: java.lang.UnsatisfiedLinkError: dlopen failed: cannot locate symbol "__extenddftf2" referenced by "/data/app/~~oWhhee07OzLL55nrtCgF1A==/com.tauri.tauri_issues_app-6AshWwk-VAC3roCMDl_dDA==/base.apk!/lib/x86_64/libtauri_issues_app_lib.so"...
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at java.lang.Runtime.loadLibrary0(Runtime.java:1082)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at java.lang.Runtime.loadLibrary0(Runtime.java:1003)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at java.lang.System.loadLibrary(System.java:1661)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at com.tauri.tauri_issues_app.WryActivity.<clinit>(WryActivity.kt:116)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at java.lang.Class.newInstance(Native Method)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.AppComponentFactory.instantiateActivity(AppComponentFactory.java:95)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at androidx.core.app.CoreComponentFactory.instantiateActivity(CoreComponentFactory.java:45)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.Instrumentation.newActivity(Instrumentation.java:1378)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.ActivityThread.performLaunchActivity(ActivityThread.java:3676)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.ActivityThread.handleLaunchActivity(ActivityThread.java:3922)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.servertransaction.LaunchActivityItem.execute(LaunchActivityItem.java:103)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.servertransaction.TransactionExecutor.executeCallbacks(TransactionExecutor.java:139)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.servertransaction.TransactionExecutor.execute(TransactionExecutor.java:96)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.ActivityThread$H.handleMessage(ActivityThread.java:2443)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.os.Handler.dispatchMessage(Handler.java:106)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.os.Looper.loopOnce(Looper.java:205)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.os.Looper.loop(Looper.java:294)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at android.app.ActivityThread.main(ActivityThread.java:8177)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at java.lang.reflect.Method.invoke(Native Method)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at com.android.internal.os.RuntimeInit$MethodAndArgsCaller.run(RuntimeInit.java:552)
02-17 11:59:38.324  3114  3114 E AndroidRuntime:        at com.android.internal.os.ZygoteInit.main(ZygoteInit.java:971)
```

## Solution
A solution will be provided once figured out or fixed.

