use chrono::prelude::*;
use pmpktn_tui::app::*;
use pmpktn_tui::sql;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::ConnectOptions;
use std::str::FromStr;



pub fn get_uri() -> String{
    let dir = sql::create_dir().unwrap();
    let test_db_path = dir.join("test_db.db");
    match std::path::Path::new(&test_db_path).exists() {
        true => std::fs::remove_file(&test_db_path).unwrap(),
        false => (),
    }
    let mut uri = "sqlite:".to_string();
    uri.push_str(test_db_path.to_str().unwrap());
    uri
}

#[tokio::test]
async fn insert_patient_test() {
    let uri = get_uri();
    let mut conn = SqliteConnectOptions::from_str(&uri).unwrap()
        .create_if_missing(true)
        .connect()
        .await.unwrap();
    let patient = Patient {
        name: "Vương Kiến Thanh".to_string(),
        is_male: true,
        birthday: Utc.ymd(1991, 02, 15),
        address: Some("142 Đường 55 Tân Tạo Bình Tân".to_string()),
        past_history: None,
    };
    let res = sql::insert_patient(&mut conn, patient).await.unwrap();
    assert_eq!(res.rows_affected(), 1);
}
