mod data_processing;
use data_processing::{read_excel_lines, CircleGakuyukaiRates, CircleInfo, GakuyukaiMembers};
use log::{debug, error, info};
use serde::Serialize;
use serde_json::Value;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use tauri::{Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};

#[derive(Serialize)]
struct GakuyukaiFile {
    path: String,
    id_line: i64,
    is_line: i64,
    timestamp: i64,
}

#[tauri::command]
fn greet(name: &str) -> String {
    print!("greet");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn wrap_load_gakuyukai_members(
    state: tauri::State<'_, GakuyukaiMembers>,
    export_path: tauri::State<'_, PathBuf>,
    path: String,
    id_line: i64,
    is_line: i64,
) -> Result<(), String> {
    debug!("wrap_load_gakuyukai_members");
    let _ = state.load_gakuyukai_members_self(&path, id_line, is_line)?;

    let export = export_path.join("gakuyukai_member.json");
    let _ = export_to_json(export, path, id_line, is_line);
    Ok(())
}

#[tauri::command]
fn wrap_get_total_member_count(state: tauri::State<'_, GakuyukaiMembers>) -> i64 {
    debug!("wrap_get_total_member_count");
    state.get_info().print();
    return state.get_gakuyukai_member_count();
}

#[tauri::command]
fn wrap_get_info(state: tauri::State<'_, GakuyukaiMembers>) -> CircleInfo {
    debug!("wrap_get_info");
    state.get_info().print();
    return state.get_info();
}

#[tauri::command]
fn wrap_calculate_gakuyukai_rate(
    state: tauri::State<'_, GakuyukaiMembers>,
    path: &str,
    id_line: i64,
    name_line: i64,
) -> Result<CircleInfo, String> {
    debug!("wrap_calculate_rakuyukai_rate");
    state
        .calculate_gakuyukai_rate(&path, id_line, name_line)?
        .print();
    return state.calculate_gakuyukai_rate(&path, id_line, name_line);
}
#[tauri::command]
fn wrap_calculate_gakuyukai_rates(
    state: tauri::State<'_, GakuyukaiMembers>,
    path: &str,
    id_line: i64,
    name_line: i64,
) -> Result<CircleGakuyukaiRates, String> {
    debug!("wrap_calculate_rakuyukai_rate");
    state
        .calculate_gakuyukai_rates(&path, id_line, name_line)?
        .print();
    return state.calculate_gakuyukai_rates(&path, id_line, name_line);
}

#[tauri::command]
fn wrap_export_to_excel(rates: CircleGakuyukaiRates, path: &str) -> Result<(), String> {
    rates
        .export_to_excel(path)
        .map_err(|e| format!("Failed to export data to Excel: {}", e))
}

fn export_to_json(
    export_path: PathBuf,
    path: String,
    id_line: i64,
    is_line: i64,
) -> Result<(), String> {
    info!(
        "Exporting to JSON: path={}, id_line={}, is_line={}, export_path={}",
        path,
        id_line,
        is_line,
        &export_path.to_str().unwrap()
    );
    // let file_path: &str = "/Users/epita/Documents/dev/gakuyukai-gui/data.json";
    let data = GakuyukaiFile {
        path,
        id_line,
        is_line,
        timestamp: chrono::Utc::now().timestamp(),
    };

    // JSONにシリアライズ
    let new_data_json = serde_json::to_value(&data).map_err(|e| {
        let msg = format!("Failed to serialize data: {}", e);
        error!("{}", msg);
        msg
    })?;
    debug!("Serialized JSON: {:?}", new_data_json);

    // dirを作成
    let parent = export_path.parent().unwrap();
    create_dir_all(parent).unwrap_or_else(|why| {
        error!("! {:?}", why.kind());
    });

    // JSONファイルを確認し、存在しない場合はスキップ
    let mut contents = String::new();
    match File::open(&export_path) {
        Ok(mut file) => {
            debug!("File exists, proceeding to read.");
            file.read_to_string(&mut contents).map_err(|e| {
                let msg = format!("Failed to read file: {}", e);
                error!("{}", msg);
                msg
            })?;
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                debug!("File does not exist, creating a new one.");
            } else {
                return Err(format!("Failed to open file: {}", e));
            }
        }
    }

    let mut json_data: Vec<Value> = if !contents.is_empty() {
        serde_json::from_str(&contents).map_err(|e| {
            let msg = format!("Failed to parse JSON: {}", e);
            error!("{}", msg);
            msg
        })?
    } else {
        Vec::new()
    };

    json_data.push(new_data_json);
    debug!("JSON data: {:?}", json_data);

    // ファイル内容を上書き
    let mut file = File::create(&export_path).map_err(|e| {
        let msg = format!("Failed to create file: {}", e);
        error!("{}", msg);
        msg
    })?;
    file.write_all(
        serde_json::to_string(&json_data)
            .map_err(|e| {
                let msg = format!("Failed to serialize JSON: {}", e);
                error!("{}", msg);
                msg
            })?
            .as_bytes(),
    )
    .map_err(|e| {
        let msg = format!("Failed to write to file: {}", e);
        error!("{}", msg);
        msg
    })?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(Target::new(TargetKind::Stdout))
                .target(Target::new(TargetKind::Webview))
                .level(log::LevelFilter::Debug)
                .format(|out, message, record| {
                    let level_color = match record.level() {
                        log::Level::Error => "\x1b[31m", // Red
                        log::Level::Warn => "\x1b[33m",  // Yellow
                        log::Level::Info => "\x1b[32m",  // Green
                        log::Level::Debug => "\x1b[34m", // Blue
                        log::Level::Trace => "\x1b[35m", // Magenta
                    };
                    let reset_color = "\x1b[0m"; // Reset

                    out.finish(format_args!(
                        "[{}{}{} {}] {}",
                        level_color,
                        record.level(),
                        reset_color,
                        record.target(),
                        message
                    ))
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            greet,
            wrap_load_gakuyukai_members,
            wrap_get_total_member_count,
            wrap_get_info,
            wrap_calculate_gakuyukai_rate,
            wrap_calculate_gakuyukai_rates,
            wrap_export_to_excel,
            read_excel_lines,
        ])
        .setup(|app| {
            let gakuyukai = GakuyukaiMembers::default();
            let app_data_dir = app.path().app_data_dir().unwrap();
            match File::open(app_data_dir.join("gakuyukai_member.json")) {
                Ok(mut file) => {
                    info!("File exists");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).map_err(|e| {
                        let msg = format!("Failed to read file: {}", e);
                        error!("{}", msg);
                        msg
                    })?;
                    debug!("File contents: {}", contents);
                    let json_data: Vec<Value> = serde_json::from_str(&contents).map_err(|e| {
                        let msg = format!("Failed to parse JSON: {}", e);
                        error!("{}", msg);
                        msg
                    })?;
                    debug!("Parsed JSON data: {:?}", json_data);
                    let latest_data = json_data.last().ok_or("No data found")?;
                    let path = latest_data["path"]
                        .as_str()
                        .ok_or("Failed to get path from JSON")?;
                    let id_line = latest_data["id_line"]
                        .as_i64()
                        .ok_or("Failed to get id_line from JSON")?;
                    let is_line = latest_data["is_line"]
                        .as_i64()
                        .ok_or("Failed to get is_line from JSON")?;
                    gakuyukai
                        .load_gakuyukai_members_self(path, id_line, is_line)
                        .map_err(|e| {
                            let msg = format!("Failed to load Gakuyukai members: {}", e);
                            error!("{}", msg);
                            msg
                        })?;
                    // フロントへイベント発行
                    app.emit("navigate", "dashboard")
                        .expect("Failed to emit event");
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        info!("File does not exist");
                    } else {
                        error!("Error opening file: {}", e);
                    }
                }
            }
            app.manage(app_data_dir);
            app.manage(gakuyukai);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
