use num_enum::TryFromPrimitiveError;
use thiserror::Error;

use crate::Errno;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Errno(#[from] Errno),

    #[error(transparent)]
    TryIntoErrno(#[from] TryFromPrimitiveError<Errno>),
}
