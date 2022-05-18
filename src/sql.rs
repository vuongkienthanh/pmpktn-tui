use crate::MyResult;
use crate::app::Patient;
use home;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{sqlite::SqliteConnection, sqlite::SqliteQueryResult, ConnectOptions};
use std::str::FromStr;
use tokio;

use std::fs;
use std::path::PathBuf;

const CREATE_DB_SQL: &str = "
CREATE TABLE patients (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    is_male INTEGER NOT NULL,
    birthday TEXT NOT NULL,
    address TEXT,
    past_history TEXT --bệnh nền
);

CREATE INDEX patient_name ON patients (name);

CREATE TABLE visits (
    id INTEGER PRIMARY KEY,
    exam_date TEXT NOT NULL,
    note TEXT, --bệnh sử
    diagnosis TEXT NOT NULL,
    weight NUMERIC,
    days INTEGER NOT NULL,
    patient_id INTEGER,
    FOREIGN KEY (patient_id)
    REFERENCES patients (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);

--Kho thuốc
CREATE TABLE warehouse (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    element TEXT NOT NULL, --thành phần thuốc
    quantity INTEGER NOT NULL, --số lượng trong kho
    usage_unit TEXT NOT NULL, --đơn vị sử dụng
    sale_unit TEXT NOT NULL, --đơn vị bán
    sale_price TEXT NOT NULL, --giá bán
    usage TEXT NOT NULL --cách sử dụng
);

--để search trong app
CREATE INDEX drug_name ON warehouse(name);
CREATE INDEX drug_element ON warehouse(element);

--Toa thuốc
CREATE TABLE linedrugs (
    id INTEGER PRIMARY KEY,
    drug_id INTEGER,
    dosage_per NUMERIC NOT NULL, --liều 1 cữ
    times INTEGER NOT NULL,--số cữ
    quantity INTEGER NOT NULL, --số lượng bán ra
    usage TEXT NOT NULL, --cách dùng
    visit_id INTEGER,
    FOREIGN KEY (visit_id)
    REFERENCES visits (id)
    ON DELETE CASCADE
    ON UPDATE CASCADE
);
";

fn create_dir() -> Result<PathBuf, std::io::Error> {
    let mut dir = home::home_dir().unwrap();
    dir.push(".pmpktn-tui");
    if !dir.exists() {
        fs::create_dir(&dir)?;
    }
    Ok(dir)
}

pub fn get_db_path() -> MyResult<PathBuf> {
    let dir = create_dir()?;
    let db_path = dir.join("mydb.db");
    Ok(db_path)
}

pub async fn get_conn(db_path: PathBuf) -> Result<SqliteConnection, sqlx::Error> {
    let mut uri = "sqlite:".to_string();
    uri.push_str(db_path.to_str().unwrap());
    SqliteConnectOptions::from_str(&uri)?
        .create_if_missing(true)
        .connect()
        .await
}

#[tokio::main]
pub async fn initdb() -> Result<SqliteQueryResult, sqlx::Error> {
    match get_db_path() {
        Ok(db_path) => {
            let mut conn = get_conn(db_path).await?;
            sqlx::query(&CREATE_DB_SQL).execute(&mut conn).await
        }
        Err(e) => Err(sqlx::Error::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Error getting db path: {}", e),
        ))),
    }
}



#[tokio::main]
pub async fn insert_patient(patient: Patient) ->Result<SqliteQueryResult, sqlx::Error> {
    todo!("asdf")
}
