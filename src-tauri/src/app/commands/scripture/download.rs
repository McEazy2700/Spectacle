use crate::{
    app::{
        config::db::get_bible_db_conn,
        state::BibleDB,
        utils::dirs::{get_app_data_dir, get_bible_dir},
    },
    common::errors::AppError,
};
use reqwest::{Client, Url};
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Cursor, Write},
};
use tauri::State;

#[tauri::command]
pub async fn download_bible(url: String) -> Result<String, AppError> {
    let file_url = match Url::parse(url.as_str()) {
        Ok(url) => Ok(url),
        Err(err) => Err(AppError::new(err.to_string().as_str())),
    }?;
    let client = Client::new();

    let name_parts: Vec<&str> = url.split("/").collect();
    let file_name = match name_parts.last() {
        Some(name) => Ok(format!("{}", name)),
        None => Err(AppError::new("Invalid file url")),
    }?;

    let data_dir = get_app_data_dir()?;
    let temp_dir = data_dir.join("temp");
    fs::create_dir_all(&temp_dir)?;
    let file_path = temp_dir.join(&file_name);

    let mut file = File::create(&file_path)?;

    let response = client.get(file_url).send().await?;
    let content = response.bytes().await?;
    file.write_all(&content)?;

    Ok(file_name)
}

#[tauri::command]
pub fn extract_bible_zip(file_name: String) -> Result<String, AppError> {
    let data_dir = get_app_data_dir()?;
    let temp_dir = data_dir.join("temp");

    let target_dir_name = file_name.split(".").next().unwrap_or("temp_zip");
    let target_dir = temp_dir.join(&target_dir_name);

    let file_path = temp_dir.join(&file_name);
    let file = fs::read(file_path)?;
    match zip_extract::extract(Cursor::new(file), &target_dir, true) {
        Ok(_) => Ok(()),
        Err(err) => Err(AppError::new(err.to_string().as_str())),
    }?;
    Ok(target_dir_name.to_string())
}

#[tauri::command]
pub fn parse_bible_sql(folder_name: String) -> Result<String, AppError> {
    let data_dir = get_app_data_dir()?;
    let temp_dir = data_dir.join("temp");
    let bible_dir = temp_dir.join(&folder_name);
    let mut first_insert = true;

    let sql_file_path = match fs::read_dir(&bible_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension() == Some("sql".as_ref()))
        .map(|entry| entry.path())
        .next()
    {
        Some(sql_file_path) => Ok(sql_file_path),
        None => Err(AppError::new("No sql file found")),
    }?;

    let sql_file = File::open(&sql_file_path)?;
    let mut reader = BufReader::new(sql_file);
    let output_file_path = bible_dir.join("bible.sql");
    let mut output_file = File::create(&output_file_path)?;

    let mut bible_table_name = String::new();

    for line in reader.lines() {
        let original_line = line?;
        if original_line.trim().starts_with("LOCK") || original_line.trim().starts_with("UNLOCK") {
            continue;
        }
        if original_line.trim().starts_with("DROP TABLE") {
            let to_replace = original_line
                .trim()
                .split_once("EXISTS")
                .unwrap_or(("", ""))
                .1;
            let modified = original_line.replace(to_replace, " bible;");
            writeln!(output_file, "{}", modified)?;
        } else if original_line.trim().starts_with("CREATE") {
            bible_table_name = original_line
                .trim()
                .split(" ")
                .nth(2)
                .unwrap_or("")
                .to_string();
            let modified = original_line.replace(&bible_table_name, "bible");
            writeln!(output_file, "{}", modified)?;
        } else if original_line.trim().starts_with("INSERT") {
            if first_insert {
                writeln!(output_file, "{}", "BEGIN TRANSACTION;")?;
                writeln!(output_file, "{}", "INSERT INTO bible VALUES")?;
                let values = original_line
                    .trim()
                    .split_once("VALUES")
                    .unwrap_or(("", ""))
                    .1
                    .replace(";", "");
                writeln!(output_file, "{}", format!("   {}", values))?;
                first_insert = false
            } else {
                let values = original_line
                    .trim()
                    .split_once("VALUES")
                    .unwrap_or(("", ""))
                    .1
                    .replace(";", "");
                writeln!(output_file, "{}", format!("   ,{}", values))?;
            }
        } else if original_line.trim().starts_with("ALTER") {
            continue;
        } else {
            // Replace the bible name and others
            let modified = original_line
                .replace(&bible_table_name, "bible")
                .replace(" ENGINE=MyISAM", "")
                .replace(" SET UTF8", "");
            writeln!(output_file, "{}", modified)?;
        }
    }
    writeln!(output_file, "{}", ";")?;
    writeln!(output_file, "{}", "COMMIT;")?;

    // Rewrite file without the first line
    let input_file = File::open(&output_file_path)?;
    reader = BufReader::new(input_file);
    let lines: Vec<String> = reader.lines().skip(1).collect::<Result<_, _>>()?;
    output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&output_file_path)?;
    for line in lines {
        writeln!(output_file, "{}", line)?;
    }

    Ok(folder_name)
}

/// Creates a database based on the bible name and writes the current temp bible.sql in the temp directory
/// to it.
#[tauri::command]
pub async fn create_bible_db(
    bible_name: String,
    temp_folder_name: String,
    state: State<'_, BibleDB>,
) -> Result<String, AppError> {
    let bible_dir = get_bible_dir()?;
    let bible_db = bible_dir.join(format!("{}.db", &bible_name));

    let data_dir = get_app_data_dir()?;
    let temp_dir = data_dir.join("temp");
    let bible_sql_path = temp_dir.join(temp_folder_name).join("bible.sql");

    let bible_sql = fs::read_to_string(bible_sql_path)?;
    File::create(bible_db)?;

    let db_conn_mutex = get_bible_db_conn(bible_name.clone(), state).await?;
    let conn = db_conn_mutex.lock().await;

    conn.execute(Statement::from_string(
        DatabaseBackend::Sqlite,
        bible_sql.as_str(),
    ))
    .await?;

    Ok(bible_name)
}

#[tauri::command]
pub fn cleanup_temp() -> Result<String, AppError> {
    let data_dir = get_app_data_dir()?;
    let temp_dir = data_dir.join("temp");
    fs::remove_dir_all(temp_dir)?;
    Ok(String::from("Cleaned up temp dir"))
}
