mod data_processing;

use data_processing::{read_excel_rows, CircleGakuyukaiRates, CircleInfo, GakuyukaiMembers};
use log::debug;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

#[tauri::command]
fn greet(name: &str) -> String {
    print!("greet");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn wrap_load_gakuyukai_members(
    state: tauri::State<'_, GakuyukaiMembers>,
    path: String,
    id_row: i64,
    is_row: i64,
) -> Result<(), String> {
    debug!("wrap_load_gakuyukai_members");
    let _ = state.load_gakuyukai_members_self(&path, id_row, is_row)?;
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
    id_row: i64,
    name_row: i64,
) -> Result<CircleInfo, String> {
    debug!("wrap_calculate_rakuyukai_rate");
    state
        .calculate_gakuyukai_rate(&path, id_row, name_row)?
        .print();
    return state.calculate_gakuyukai_rate(&path, id_row, name_row);
}
#[tauri::command]
fn wrap_calculate_gakuyukai_rates(
    state: tauri::State<'_, GakuyukaiMembers>,
    path: &str,
    id_row: i64,
    name_row: i64,
) -> Result<CircleGakuyukaiRates, String> {
    debug!("wrap_calculate_rakuyukai_rate");
    state
        .calculate_gakuyukai_rates(&path, id_row, name_row)?
        .print();
    return state.calculate_gakuyukai_rates(&path, id_row, name_row);
}

#[tauri::command]
fn wrap_export_to_excel(rates: CircleGakuyukaiRates, path: &str) -> Result<(), String> {
    rates
        .export_to_excel(path)
        .map_err(|e| format!("Failed to export data to Excel: {}", e))
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
            read_excel_rows,
        ])
        .setup(|app| {
            let gakuyukai = GakuyukaiMembers::default();
            app.manage(gakuyukai);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
