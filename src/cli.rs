use std::io::Write;

use crate::sql::get_db_path;
use crate::sql::initdb;
use crate::tui::start;
use crate::MyResult;
use clap::command;
use clap::Command as ClapCommand;

pub fn run() -> MyResult<()> {
    let m = command!()
        .propagate_version(true)
        .subcommand_required(false)
        .subcommand(ClapCommand::new("initdb").about("Initialize a new database"))
        .subcommand(ClapCommand::new("start").about("Start TUI"))
        .get_matches();

    match m.subcommand() {
        Some(("initdb", _)) => create_db(),
        Some(("start", _)) | None => start(),
        _ => Err("error".into()),
    }
}

fn create_db() -> MyResult<()> {
    match initdb() {
        Ok(_) => {
            println!("Create database successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Fail creating database: {}", e);
            print!("Remove database file and initdb again? [Y/N]");
            std::io::stdout().flush()?;
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf)?;
            match buf.trim() {
                "Y" | "y" => {
                    std::fs::remove_file(get_db_path()?).unwrap();
                    create_db()
                }
                _ => {
                    Err("Aborting".into())
                }
            }
        }
    }
}
