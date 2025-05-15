mod data_processing;
use data_processing::{read_excel_lines, CircleGakuyukaiRates, CircleInfo, GakuyukaiMembers};
use log::{debug, info};
use serde::Serialize;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::{Read, Write};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
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
    path: String,
    id_line: i64,
    is_line: i64,
) -> Result<(), String> {
    debug!("wrap_load_gakuyukai_members");
    let _ = state.load_gakuyukai_members_self(&path, id_line, is_line)?;
    let _ = export_to_json(path, id_line, is_line);
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

fn export_to_json(path: String, id_line: i64, is_line: i64) -> Result<(), String> {
    info!(
        "Exporting to JSON: path={}, id_line={}, is_line={}",
        path, id_line, is_line
    );
    let file_path: &str = "/Users/epita/Documents/dev/gakuyukai-gui/data.json";
    let data = GakuyukaiFile {
        path,
        id_line,
        is_line,
        timestamp: chrono::Utc::now().timestamp(),
    };

    // JSONにシリアライズ
    let new_data_json =
        serde_json::to_value(&data).map_err(|e| format!("Failed to serialize data: {}", e))?;

    // JSONファイルを確認し、存在しない場合は新しく作成
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut json_data: Vec<Value> = if !contents.is_empty() {
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {}", e))?
    } else {
        Vec::new()
    };

    json_data.push(new_data_json);

    // ファイル内容を上書き
    file.set_len(0)
        .map_err(|e| format!("Failed to truncate file: {}", e))?;
    file.write_all(
        serde_json::to_string(&json_data)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?
            .as_bytes(),
    )
    .map_err(|e| format!("Failed to write to file: {}", e))?;

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
                .level(log::LevelFilter::Info)
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
            app.manage(gakuyukai);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
