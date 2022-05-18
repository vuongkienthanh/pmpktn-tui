use chrono::prelude::*;

pub struct Patient {
    name: String,
    is_male: bool,
    birthday: DateTime<Utc>,
    address: String,
    past_history: String,
}
