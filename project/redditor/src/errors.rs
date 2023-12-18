use std::{io, path::PathBuf};
use thiserror::Error;
use ureq;
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Could not fetch data from the subreddit")]
    RequestError(String, ureq::Error),
    #[error("Could not access current directory")]
    CurrentDirectoryError(io::Error),
    #[error("Could not access the executables parent directory")]
    BackupDirectoryError,
    #[error("Received invalid JSON")]
    JSONError(#[from] serde_json::Error),
    #[error("Could not find subreddit with url:{0}")]
    InvalidSubredditError(String),
    #[error("Could not read from file {0}")]
    FileReadError(PathBuf, io::Error),
    #[error("Could not copy backup file")]
    CopyFileError(io::Error),
    #[error("Could not remove cloned backup file")]
    RemoveFileError(io::Error),
    #[error("Could not create backup file")]
    CreateFileError(io::Error),
    #[error("Could not write to backup file")]
    WriteFileError(io::Error),
    #[error("Could not convert response to string")]
    StringConversionError(io::Error),
}
