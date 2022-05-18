use crate::tui::start;
use std::env;
use crate::MyResult;
use clap::{command, Arg, Command as ClapCommand};
use std::fs;
use home;
use std::io::{self, Write};
use std::process::Command as SysCommand;

pub fn run() -> MyResult<()> {
    let m = command!()
        .propagate_version(true)
        .subcommand_required(false)
        .subcommand(ClapCommand::new("initdb").about("Initialize a new database"))
        .subcommand(
            ClapCommand::new("insert").about("Insert data").arg(
                Arg::new("patient")
                    .help("patient_name")
                    .long("patient")
                    .short('p').takes_value(true),
            ),
        )
        .subcommand(ClapCommand::new("start").about("Start TUI"))
        .get_matches();

    match m.subcommand() {
        Some(("initdb", _)) => create_db(),
        Some(("insert", subm)) => {
            println!("insert patient name {}", env::home_dir().unwrap().display());
            Ok(())
        }
        Some(("start", _)) | None => start(),
        _ => {
            eprintln!("Error");
            Err("error".into())
        }
    }
}

fn create_db() -> MyResult<()> {
    let sql = "
CREATE TABLE IF NOT EXISTS patients (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  gender INTEGER NOT NULL,
  birthday TEXT NOT NULL,
  address TEXT,
  past_history TEXT --bệnh nền
);

CREATE INDEX IF NOT EXISTS patient_name
  ON patients (name);

CREATE TABLE IF NOT EXISTS visits (
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
CREATE TABLE IF NOT EXISTS warehouse (
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
CREATE INDEX IF NOT EXISTS drug_name
  ON warehouse(name);
CREATE INDEX IF NOT EXISTS drug_element 
  ON warehouse(element);

--Toa thuốc
CREATE TABLE IF NOT EXISTS linedrugs (
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

    fs::create_dir_all("~/.pmpktn-tui")?;

    let db_name = "~/.pmpktn-tui/mydb.db";
    let cmd = SysCommand::new("sqlite3").arg(db_name).arg(sql).output();
    match cmd {
        Ok(output) => {
            println!("Succeed in executing command: {}", output.status);
            println!("database file: {}", db_name);
            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
            Ok(())
        }
        Err(e) => Err(format!("Error executing command {}", e).into()),
    }
}
