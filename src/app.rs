use chrono::prelude::*;

pub struct Patient {
    pub name: String,
    pub is_male: bool,
    pub birthday: Date<Utc>,
    pub address: Option<String>,
    pub past_history: Option<String>,
}
