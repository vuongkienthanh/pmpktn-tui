pub type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub mod cli;
pub mod tui;
