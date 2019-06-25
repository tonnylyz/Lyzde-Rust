use std::fmt;
use std::fmt::Formatter;

pub enum DbHelperError {
    MySqlError,
    OptionNoneError,
}

type Result<T> = std::result::Result<T, DbHelperError>;

impl fmt::Display for DbHelperError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self {
            DbHelperError::MySqlError => { write!(f, "MySqlError") },
            DbHelperError::OptionNoneError => { write!(f, "InvalidResultError") },
        }
    }
}

impl std::convert::From<mysql::Error> for DbHelperError {
    fn from(_: mysql::Error) -> Self {
        DbHelperError::MySqlError
    }
}

impl std::convert::From<std::option::NoneError> for DbHelperError {
    fn from(_: std::option::NoneError) -> Self {
        DbHelperError::OptionNoneError
    }
}

pub fn query_single<T>(cmd : &str) -> Result<T> where T: mysql::prelude::FromValue {
    let pool = mysql::Pool::new(crate::config::conn())?;
    let row = pool.first_exec(cmd, ())??;
    Ok(row.get::<T, usize>(0)?)
}
