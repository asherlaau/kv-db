use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Errors {
    #[error("failed to read from data file")]
    FailedToReadFromDataFile,

    #[error("failed to write to data file")]
    FailedWriteToDataFile,

    #[error("failed to Syncronize data file")]
    FailedSyncDataFile,

    #[error("failed to open data file")]
    FailedOpenDataFile,
}

pub type Result<T> = result::Result<T, Errors>;
