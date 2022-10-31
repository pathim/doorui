use std::fmt::Display;

pub mod mysql;
pub mod sqlite;

#[derive(Debug, Default)]
pub struct Card {
    pub uid: String,
    pub pin: String,
    pub owner: String,
    pub member_id: String,
    pub expiry: chrono::NaiveDate,
    pub allow_unlock: bool,
    pub allow_entry: bool,
    pub allow_sneaky: bool,
    pub always_sneaky: bool,
}

#[derive(Debug)]
pub struct Log {
    pub id: u32,
    pub date: chrono::DateTime<chrono::Utc>,
    pub card_uid: String,
    pub req_type: String,
    pub door_name: String,
}

#[derive(Debug)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait Db {
    fn get_cards(&self) -> Result<Vec<Card>>;
    fn insert_card(&self, card: Card) -> Result<()>;
    fn remove_card(&self, uid: String) -> Result<()>;
    fn get_logs_success(&self) -> Result<Vec<Log>>;
    fn get_logs_failure(&self) -> Result<Vec<Log>>;
}
