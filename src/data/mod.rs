pub mod model;


use postgres::{Client, error::Error as PgError, row::Row, types::ToSql};
use super::config::Config;
use std::sync::Mutex;
pub trait DBConnection: Send + Sync {
    // TODO: use simple envelop in the interface instead of postgres::row::Row and dyn ToSql
    fn query_get_each(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, DbError>;
    fn query_get(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, DbError>;
    fn query_edit(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, DbError>;
}

#[derive(Debug)]
pub enum DbError {
    Wrong(String),
    Unknown
}

pub struct PostgresEnvelop {
    connection: Mutex<Client>
}

impl PostgresEnvelop {
    pub fn init(cfg: &Config) -> Result<Self, PgError> {
        let connection = Client::connect(&cfg.db_connect, postgres::tls::NoTls)?;
        Ok(Self{
            connection: Mutex::new(connection),
        })
    }
}

// Postgres implementation
impl DBConnection for PostgresEnvelop {
    // Use to take data
    fn query_get_each(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Row>, DbError> {
        let mut conn = self.connection.lock().unwrap();
        let res: Vec<Row> = conn.query(query, params)?;
        Ok(res)
    }

    // Use to take a single result
    fn query_get(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Row, DbError> {
        let mut conn = self.connection.lock().unwrap();
        let res: Row = conn.query_one(query, params)?;
        Ok(res)
    }

    // Use to edit data
    fn query_edit(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<u64, DbError> {
        let mut conn = self.connection.lock().unwrap();
        let res: u64 = conn.execute(query, params)?;
        Ok(res)
    }
}

unsafe impl Send for PostgresEnvelop{}
unsafe impl Sync for PostgresEnvelop{}

impl From<PgError> for DbError {
    fn from(f: PgError) -> DbError {
        DbError::Wrong(f.to_string())
    }
}
