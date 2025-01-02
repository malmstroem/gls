use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    #[from]
    Std(#[serde_as(as = "DisplayFromStr")] std::io::Error),
    #[from]
    Serde(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
    #[from]
    Iwf(#[serde_as(as = "DisplayFromStr")] iwf::Error),
    #[from]
    IwfSql(#[serde_as(as = "DisplayFromStr")] iwf::sql::Error),
    #[from]
    Csv(#[serde_as(as = "DisplayFromStr")] csv::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
