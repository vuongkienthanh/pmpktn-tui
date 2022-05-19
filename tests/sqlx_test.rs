use chrono::prelude::*;
use pmpktn_tui::app::*;
use pmpktn_tui::sql;
use sqlx::Connection;

pub async fn setup() -> sqlx::sqlite::SqliteConnection {
    let dir = sql::create_dir().unwrap();
    let test_db_path = dir.join("test_db.db");
    let mut uri = "sqlite:".to_string();
    uri.push_str(test_db_path.to_str().unwrap());
    sqlx::SqliteConnection::connect(&uri).await.unwrap()
}

#[tokio::test]
async fn insert_patient_test() {
    let mut conn = setup();
    let patient = Patient {
        name: "Vương Kiến Thanh".to_string(),
        is_male: true,
        birthday: Utc.ymd(1991, 02, 15),
        address: Some("142 Đường 55 Tân Tạo Bình Tân".to_string()),
        past_history: None,
    };
    sql::insert_patient(&mut conn, patient).unwrap_or_else(f);
}
