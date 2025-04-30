use calamine::{open_workbook_auto, DataType, Reader};
use cli_table::{format::Justify, print_stdout, Cell, Table, WithTitle};
use log::{debug, error, info, warn};
use rayon::prelude::*;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, XlsxError};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{
    fs,
    io::{self},
    path::{Path, PathBuf},
};

// Students構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct Students {
    students: Vec<Student>,
}

// Student構造体
#[derive(Debug, Table, Serialize, Deserialize)]
struct Student {
    #[table(title = "学籍番号")]
    id: i64,
    #[table(title = "名前")]
    name: String,
    #[table(skip)]
    is_gakuyukai: Option<bool>,
}

// CircleGakuyukaiRates構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct CircleGakuyukaiRates {
    circles: Vec<CircleInfo>,
    error_file_path: Vec<String>,
}

// CircleInfo構造体
#[derive(Debug, Table, Serialize, Deserialize)]
pub struct CircleInfo {
    #[table(skip)]
    file_path: String,
    #[table(title = "サークル名")]
    name: String,
    #[table(skip)]
    rate: f64,
    #[table(title = "学友会員率")]
    rate_string: String,
    #[table(title = "メンバー数")]
    member_count: i64,
    #[table(title = "学友会員数")]
    gakuyukai_member_count: i64,
    #[table(skip)]
    member: Option<Students>,
}

// GakuyukaiMembers構造体
#[derive(Debug, Serialize, Deserialize)]
pub struct GakuyukaiMembers {
    member_ids: Mutex<Vec<i64>>,
    non_member_ids: Mutex<Vec<i64>>,
}

impl Default for GakuyukaiMembers {
    fn default() -> Self {
        GakuyukaiMembers {
            member_ids: Mutex::new(vec![]),
            non_member_ids: Mutex::new(vec![]),
        }
    }
}
impl GakuyukaiMembers {
    /// 学友会情報を返す
    pub fn get_info(&self) -> CircleInfo {
        debug!("info for GakuyukaiMembers");
        return CircleInfo {
            name: "学友会".to_string(),
            file_path: "".to_string(),
            rate: self.get_gakuyukai_rate(),
            member_count: self.get_total_member_count(),
            gakuyukai_member_count: self.get_gakuyukai_member_count(),
            rate_string: format!("{:.2}%", self.get_gakuyukai_rate() * 100_f64),
            member: None,
        };
    }
    /// 学友会名簿全体の人数を返す
    pub fn get_total_member_count(&self) -> i64 {
        debug!("Calculating total member count for GakuyukaiMembers");
        let member_count = {
            let member_ids_lock = self.member_ids.lock().unwrap();
            member_ids_lock.len() as i64
        };
        let non_member_count = {
            let non_member_ids_lock = self.non_member_ids.lock().unwrap();
            non_member_ids_lock.len() as i64
        };
        member_count + non_member_count
    }
    /// 学友会名簿の中の学友会員数を返す
    pub fn get_gakuyukai_member_count(&self) -> i64 {
        debug!("Calculating gakuyukai member count for GakuyukaiMembers");
        let member_ids_lock = self.member_ids.lock().unwrap();
        member_ids_lock.len() as i64
    }
    /// 学友会名簿の学友会員率を返す
    pub fn get_gakuyukai_rate(&self) -> f64 {
        debug!("Calculating gakuyukai rate for GakuyukaiMembers");
        let member_count = {
            let member_ids = self.member_ids.lock().unwrap();
            member_ids.len() as f64
        };
        let total_count = {
            let non_member_ids = self.non_member_ids.lock().unwrap();
            member_count + non_member_ids.len() as f64
        };
        if total_count == 0.0 {
            return 0.0; // Total count is zero, avoid division by zero
        }
        member_count / total_count
    }

    // const ID_ROW: i64 = 0;
    // const IS_ROW: i64 = 2;
    /// 学友会メンバーのリストをExcelファイルから読み込む関数
    pub fn load_gakuyukai_members(path: &str, id_row: i64, is_row: i64) -> Result<Self, String> {
        info!("Loading gakuyukai members from file: {}", path);

        let mut member_ids: Vec<i64> = Vec::new();
        let mut non_member_ids: Vec<i64> = Vec::new();
        let mut workbook = read_excel_file(&path)?;
        let sheet_names = workbook.sheet_names();
        let sheet_name = sheet_names.first().unwrap();
        debug!("Processing sheet: {}", sheet_name);
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            debug!("range.width: {}", range.width());
            if range.width() > id_row as usize && range.width() > is_row as usize {
                for row in range.rows() {
                    let student_id: Option<i64> = row[id_row as usize].as_i64();
                    let is_gakuyukai_member: Option<bool> = row[is_row as usize].get_bool();
                    if student_id.is_some_and(|id| (1_000_000..=9_999_999).contains(&id)) {
                        if is_gakuyukai_member.is_some_and(|g| g) {
                            member_ids.push(student_id.unwrap());
                        } else {
                            non_member_ids.push(student_id.unwrap());
                        }
                    }
                }
            } else {
                return Err("Column width is shorter than expected".to_string());
            }
        }
        if member_ids.iter().count() == 0 || non_member_ids.iter().count() == 0 {
            return Err("No one here".to_string());
        }
        info!(
            "Loaded {} members and {} non-members.",
            member_ids.len(),
            non_member_ids.len()
        );
        return Ok(GakuyukaiMembers {
            member_ids: Mutex::new(member_ids),
            non_member_ids: Mutex::new(non_member_ids),
        });
    }

    pub fn load_gakuyukai_members_self(
        &self,
        path: &str,
        id_row: i64,
        is_row: i64,
    ) -> Result<(), String> {
        // Attempt to load the gakuyukai members from the given path
        match GakuyukaiMembers::load_gakuyukai_members(path, id_row, is_row) {
            Ok(gakuyukai) => {
                // Acquire the lock and update the member_ids
                let mut self_gakuyukai_id = self.member_ids.lock().unwrap();
                *self_gakuyukai_id = gakuyukai.member_ids.lock().unwrap().clone();

                let mut self_non_member_ids = self.non_member_ids.lock().unwrap();
                *self_non_member_ids = gakuyukai.non_member_ids.lock().unwrap().clone();

                Ok(())
            }
            Err(e) => {
                // Handle the error gracefully
                error!("Failed to load gakuyukai members from '{}': {}", path, e);
                Err(e)
            }
        }
    }

    pub fn calculate_gakuyukai_rate(&self, path: &str) -> Result<CircleInfo, String> {
        info!("Calculating gakuyukai rate for file: {}", path);

        // ファイル形式のチェック
        if !check_excel_file_path(&path) {
            let error_message = format!("Skipping non-Excel file: {}", path);
            error!("{}", error_message);
            return Err(error_message);
        }

        // サークルメンバーを読み込む
        let circle_members: Vec<Student> = Students::load_circle_members(path)?
            .students
            .iter()
            .map(|s| Student {
                id: s.id,
                name: s.name.clone(),
                is_gakuyukai: Some(self.member_ids.lock().expect("REASON").contains(&s.id)),
            })
            .collect::<Vec<Student>>();

        // 学友会メンバーの数を計算
        let gakuyukai_count = circle_members
            .iter()
            .filter(|s| s.is_gakuyukai.unwrap_or(false))
            .count();
        let total_count = circle_members.len();

        if total_count == 0 {
            let error_message = format!("No students to calculate rate in file: {}", path);
            error!("{}", error_message);
            return Err(error_message);
        }

        // 割合の計算
        let rate = gakuyukai_count as f64 / total_count as f64;

        // ログに出力
        info!(
            "Calculated gakuyukai rate for {}: {:.2}%",
            path,
            rate * 100.0
        );

        // CircleInfoを生成
        let circle_info = CircleInfo {
            file_path: path.to_string(),
            name: Path::new(&path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string(),
            rate,
            member_count: total_count as i64,
            gakuyukai_member_count: gakuyukai_count as i64,
            rate_string: format!("{:.2}%", rate * 100_f64),
            member: Some(Students {
                students: circle_members,
            }),
        };

        Ok(circle_info)
    }

    /// 指定されたディレクトリ内の各ファイルについて学友会加入率を計算する関数
    pub fn calculate_gakuyukai_rates(
        &self,
        directory_path: &str,
    ) -> Result<CircleGakuyukaiRates, String> {
        info!(
            "Calculating gakuyukai rates for directory: {}",
            directory_path
        );
        debug!("Using members: {:?}", self.member_ids);

        let mut gakuyukai_rates = CircleGakuyukaiRates::new();
        let excel_files: Vec<PathBuf> = read_dir_entries(directory_path)
            .map_err(|e| format!("Failed to read directory {}: {:?}", directory_path, e))?;

        // ここから並列化！
        let results: Vec<(String, Result<CircleInfo, String>)> = excel_files
            .par_iter()
            .map(|path| {
                let file_path = path.to_string_lossy().into_owned();
                if check_excel_file_path(&file_path) {
                    debug!("Processing file: {}", file_path);
                    let result = self.calculate_gakuyukai_rate(&file_path);
                    (file_path, result)
                } else {
                    warn!("Skipping non-Excel file: {}", &file_path);
                    (
                        file_path.clone(),
                        Err(format!("Skipping non-Excel file: {}", &file_path)),
                    )
                }
            })
            .collect();

        // 並列で出た結果を集める
        for (file_path, result) in results {
            match result {
                Ok(info) => {
                    gakuyukai_rates.push(info);
                }
                Err(err) => {
                    warn!("{}", err);
                    gakuyukai_rates.push_error_file_path(file_path);
                }
            }
        }

        info!("Completed calculating rates for all files.");
        Ok(gakuyukai_rates)
    }

    //     pub fn calculate_gakuyukai_rates(
    //         &self,
    //         directory_path: &str,
    //     ) -> Result<CircleGakuyukaiRates, String> {
    //         info!(
    //             "Calculating gakuyukai rates for directory: {}",
    //             directory_path
    //         );
    //         debug!("Using members: {:?}", self.member_ids);

    //         let mut gakuyukai_rates: CircleGakuyukaiRates = CircleGakuyukaiRates::new();
    //         let excel_files: Vec<PathBuf> = read_dir_entries(directory_path)
    //             .map_err(|e| format!("Failed to read directory {}: {:?}", directory_path, e))?;
    //         for file_path in excel_files
    //             .iter()
    //             .map(|path| path.to_string_lossy().into_owned())
    //         {
    //             if check_excel_file_path(&file_path) {
    //                 debug!("Processing file: {}", file_path);
    //                 let _result: Result<_, _> = self
    //                     .calculate_gakuyukai_rate(&file_path)
    //                     .and_then(|info: CircleInfo| {
    //                         gakuyukai_rates.push(info);
    //                         return Ok(());
    //                     })
    //                     .or_else(|err: String| {
    //                         warn!("{}", err);
    //                         gakuyukai_rates.push_error_file_path(file_path);
    //                         return Err(err);
    //                     });
    //             } else {
    //                 warn!("Skipping non-Excel file: {}", file_path);
    //                 gakuyukai_rates.push_error_file_path(file_path);
    //                 continue;
    //             }
    //         }
    //         info!("Completed calculating rates for all files.");
    //         return Ok(gakuyukai_rates);
    // }
}

impl Students {
    const ID_ROW: i64 = 3;
    const NAME_ROW: i64 = 4;
    // const ID_ROW: i64 = 0;
    // const NAME_ROW: i64 = 1;

    /// サークルメンバーの情報をExcelファイルから読み込む関数
    fn load_circle_members(path: &str) -> Result<Self, String> {
        info!("Loading circle members from file: {}", path);

        let mut members: Vec<Student> = Vec::new();
        let mut workbook = read_excel_file(path)?;
        let sheet_names = workbook.sheet_names();
        let sheet_name = sheet_names.first().unwrap();
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            if range.width() > Students::ID_ROW as usize
                && range.width() > Students::NAME_ROW as usize
            {
                for row in range.rows() {
                    let student_id: Option<i64> = row[Students::ID_ROW as usize].as_i64();
                    let student_name: Option<String> = row[Students::NAME_ROW as usize].as_string();
                    if student_id.is_some_and(|id| (1_000_000..=9_999_999).contains(&id))
                        && student_name.is_some()
                    {
                        members.push(Student {
                            id: student_id.unwrap(),
                            name: student_name.unwrap(),
                            is_gakuyukai: None,
                        });
                    }
                }
            } else {
                return Err("Column width is shorter than expected".to_string());
            }
        }
        info!("Loaded {} circle members.", members.len());
        return Ok(Students { students: members });
    }

    pub fn print(&self) {
        // assert!(print_stdout(self.students.with_title()).is_ok());
        let members = self
            .students
            .iter()
            .map(|f| {
                vec![
                    f.id.cell(),
                    f.name.clone().cell(),
                    (if f.is_gakuyukai.unwrap() { "○" } else { "×" })
                        .cell()
                        .justify(Justify::Center),
                ]
            })
            .collect::<Vec<_>>()
            .table()
            .title(vec!["学籍番号".cell(), "名前".cell(), "学友会".cell()]);
        assert!(print_stdout(members).is_ok());
    }
}

impl CircleGakuyukaiRates {
    /// Excelファイルとして出力する
    pub fn export_to_excel(&self, path: &str) -> Result<(), XlsxError> {
        info!("Exporting CircleGakuyukaiRates to Excel: {}", path);

        // 新しいExcelファイルを作成
        let mut workbook = Workbook::new();

        // サマリーシートを作成
        let summary = workbook.add_worksheet();
        summary.set_name("サマリー")?;

        // ヘッダー用のフォーマットを設定
        let header_format = Format::new().set_bold().set_align(FormatAlign::Center);

        // パーセンテージ用のフォーマットを設定
        let percent_format = Format::new()
            .set_num_format("0.00%")
            .set_align(FormatAlign::Center);

        // 中央揃えフォーマット
        let center_format = Format::new().set_align(FormatAlign::Center);

        // サマリーシートのヘッダーを書き込む
        summary.write_with_format(0, 0, "サークル名", &header_format)?;
        summary.write_with_format(0, 1, "学友会員率", &header_format)?;
        summary.write_with_format(0, 2, "メンバー数", &header_format)?;
        summary.write_with_format(0, 3, "学友会員数", &header_format)?;

        // サマリーシートにデータを書き込む
        for (i, circle) in self.circles.iter().enumerate() {
            let row = (i + 1) as u32;
            summary.write(row, 0, &circle.name)?;
            summary.write_with_format(row, 1, circle.rate, &percent_format)?;
            summary.write_with_format(row, 2, circle.member_count as i32, &center_format)?;
            summary.write_with_format(
                row,
                3,
                circle.gakuyukai_member_count as i32,
                &center_format,
            )?;
        }

        // サマリーシートの列幅を調整
        summary.set_column_width(0, 30)?; // サークル名
        summary.set_column_width(1, 15)?; // 学友会員率
        summary.set_column_width(2, 15)?; // メンバー数
        summary.set_column_width(3, 15)?; // 学友会員数

        // 各サークルのメンバー情報を別シートに書き込む
        for (_i, circle) in self.circles.iter().enumerate() {
            if let Some(members) = &circle.member {
                let member_sheet = workbook.add_worksheet();
                member_sheet.set_name(&circle.name)?;

                // メンバーシートのヘッダー
                member_sheet.write_with_format(0, 0, "学籍番号", &header_format)?;
                member_sheet.write_with_format(0, 1, "名前", &header_format)?;
                member_sheet.write_with_format(0, 2, "学友会", &header_format)?;

                // メンバーデータを書き込む
                for (j, student) in members.students.iter().enumerate() {
                    let row = (j + 1) as u32;
                    member_sheet.write_with_format(row, 0, student.id as i32, &center_format)?;
                    member_sheet.write(row, 1, &student.name)?;
                    member_sheet.write_with_format(
                        row,
                        2,
                        student.is_gakuyukai.unwrap_or(false) as bool,
                        &center_format,
                    )?;
                }

                // メンバーシートの列幅を調整
                member_sheet.set_column_width(0, 15)?; // 学籍番号
                member_sheet.set_column_width(1, 30)?; // 名前
                member_sheet.set_column_width(2, 10)?; // 学友会
            }
        }

        // ファイルを保存
        workbook.save(path)?;

        info!("Successfully exported to Excel file");
        Ok(())
    }

    /// CSVファイルとして出力する
    #[allow(dead_code)]
    pub fn export_to_csv(&self, path: &str) -> Result<(), String> {
        info!("Exporting CircleGakuyukaiRates to CSV: {}", path);

        let mut csv_content = String::from("サークル名,学友会員率,メンバー数,学友会員数\n");

        for circle in &self.circles {
            csv_content.push_str(&format!(
                "{},{:.2}%,{},{}\n",
                circle.name, circle.rate, circle.member_count, circle.gakuyukai_member_count
            ));
        }

        fs::write(path, csv_content).map_err(|e| format!("Failed to write CSV file: {}", e))?;

        info!("Successfully exported to CSV file");
        Ok(())
    }

    /// 新規作成
    pub fn new() -> Self {
        CircleGakuyukaiRates {
            circles: Vec::new(),
            error_file_path: Vec::new(),
        }
    }

    /// サークルリストにサークル情報を追加
    pub fn push(&mut self, info: CircleInfo) {
        debug!("Pushing new CircleGakuyukaiRate: {:?}", info);
        self.circles.push(info);
    }

    /// エラーリストに追加
    pub fn push_error_file_path(&mut self, path: String) {
        debug!("Pushing new error file path: {}", path);
        self.error_file_path.push(path);
    }

    pub fn print(&self) {
        info!("Printing CircleGakuyukaiRates.");
        println!("These files were skipped");
        self.error_file_path
            .iter()
            .for_each(|f| println!("- {}", f));
        // assert!(print_stdout(self.circles.with_title()).is_ok());
    }
}

impl CircleInfo {
    pub fn print(&self) {
        assert!(print_stdout(vec![self].with_title()).is_ok());
    }

    pub fn print_member(&self) {
        self.member.as_ref().unwrap().print();
    }
}

// ディレクトリのパスからファイルを列挙する
fn read_dir_entries<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    debug!("Reading directory entries from: {:?}", path.as_ref());

    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    return Ok(entries);
}

fn read_excel_file(path: &str) -> Result<calamine::Sheets<io::BufReader<fs::File>>, String> {
    if check_excel_file_path(&path) {
        debug!("Processing file: {}", path);
        let workbook = open_workbook_auto(&path)
            .map_err(|_| format!("Cannot open file {}", path).to_string())?;
        return Ok(workbook);
    } else {
        let message: String =
            format!("Cannot open file {}. This path is not an Excel file.", path).to_string();
        warn!("{}", message);
        return Err(message);
    }
}

pub fn check_excel_file_path(path: &str) -> bool {
    if (path.ends_with(".xlsx") || path.ends_with(".xls")) && !path.contains("~$") {
        return true;
    } else {
        return false;
    }
}

#[tauri::command]
pub fn read_excel_rows(path: &str, num_rows: usize) -> Result<Vec<Vec<String>>, String> {
    debug!("Reading rows from Excel file: {}", path);

    if !check_excel_file_path(path) {
        let message: String =
            format!("Cannot open file {}. This path is not an Excel file.", path).to_string();
        return Err(message);
    }

    // Attempt to open the workbook
    let mut workbook =
        open_workbook_auto(path).map_err(|_| format!("Failed to open file: {}", path))?;

    // Obtain the first sheet's name
    let sheet_names = workbook.sheet_names();
    let sheet_name = sheet_names
        .first()
        .ok_or("No sheets found in the Excel file")?;

    debug!("Reading up to {} rows from sheet: {}", num_rows, sheet_name);

    // Attempt to obtain the range of data from the specified sheet
    let range = workbook
        .worksheet_range(sheet_name)
        .map_err(|_| format!("Failed to read the range of sheet: {}", sheet_name))?;

    // Collect the rows up to the specified number
    let rows: Vec<Vec<String>> = range
        .rows()
        .take(num_rows)
        .map(|row| row.to_vec().iter().map(|cell| cell.to_string()).collect())
        .collect();

    // Check if any rows were read
    if rows.is_empty() {
        return Err("No data found in the specified rows".to_string());
    }

    Ok(rows)
}
