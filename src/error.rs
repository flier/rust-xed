use thiserror::Error;

use crate::Errno;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Errno(#[from] Errno),

    #[error("unexpected expression, {0}")]
    UnexpectedExpr(String),

    #[error("parse encode expression failed, {0:?}")]
    Parse(nom::error::ErrorKind),
}

impl From<Errno> for Result<()> {
    fn from(err: Errno) -> Self {
        if err.is_none() {
            Ok(())
        } else {
            Err(err.into())
        }
    }
}

pub trait ToResult {
    fn result(self) -> Result<()>;
}

impl ToResult for ffi::xed_error_enum_t {
    fn result(self) -> Result<()> {
        Errno::from(self).into()
    }
}
