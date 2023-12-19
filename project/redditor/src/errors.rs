use core::fmt;
use std::{io, path::PathBuf};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Could not fetch data from the subreddit")]
    Request(#[from] Box<ureq::Error>),
    #[error("Could not access current directory")]
    CurrentDirectory(io::Error),
    #[error("Could not access the executables parent directory")]
    BackupDirectory,
    #[error("Received invalid JSON")]
    Json(#[from] serde_json::Error),
    #[error("Could not find subreddit with url:{0}")]
    InvalidSubreddit(String),
    #[error("Could not read from file {0}")]
    FileRead(PathBuf, io::Error),
    #[error("Could not copy backup file")]
    CopyFile(io::Error),
    #[error("Could not remove cloned backup file")]
    RemoveFile(io::Error),
    #[error("Could not create backup file")]
    CreateFile(io::Error),
    #[error("Could not write to backup file")]
    WriteFile(io::Error),
    #[error("Could not convert response to string")]
    StringConversion(io::Error),
    #[error("Could not display data")]
    Display(#[from] fmt::Error),
}
