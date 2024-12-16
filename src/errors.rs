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

    #[error("Key is empty")]
    KeyIsEmpty,

    #[error("in memory index updat failed")]
    IndexerUpdateFailed,

    #[error("key not found ")]
    KeyNotFound,

    #[error("data file not found")]
    DataFileNotFound,

    #[error("dirpath cannot be empty")]
    DirPathIsEmpty,

    #[error("database data file size must be greater than 0")]
    DataFileSizeTooSmall,

    #[error("failed to create the database directory")]
    FailedToCreateDatabaseDir,

    #[error("failed to read the database directory")]
    FailedToReadDatabaseDir,

    #[error("the databse directory maybe corrupted")]
    DataDirectoryCorrupted,


    #[error("read data file eof")]
    ReadDataFileEOF,
}

pub type Result<T> = result::Result<T, Errors>;
