use crate::errors::MyError;

use chrono::Local;
use serde_derive::Deserialize;
use std::collections::HashSet;
use std::env;

use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Respone {
    data: RedditData,
}
#[derive(Debug, Deserialize)]
struct RedditData {
    children: Vec<Post>,
}
#[derive(Debug, Deserialize)]
struct Post {
    data: PostData,
}
#[derive(Debug, Deserialize)]
struct PostData {
    title: String,
    author: String,
    #[serde(rename = "permalink")]
    url: String,
    #[serde(rename = "selftext")]
    text: String,
    id: String,
}
impl Display for PostData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title:{}\n Author:{}\n Text:{}\n Url:https://www.reddit.com{}\n",
            self.title, self.author, self.text, self.url
        )
    }
}
#[derive(Debug)]
pub struct SubredditUpdate {
    posts: HashSet<String>,
    url: String,
    backup_file_path: PathBuf,
    clone_file_path: PathBuf,
}

impl SubredditUpdate {
    pub fn new(subreddit: &str, order: &str) -> Result<Self, MyError> {
        let url = format!("https://www.reddit.com/r/{subreddit}/{order}.json");
        let mut backup_file_path;
        if let Some(dir) = env::current_exe()
            .map_err(|err| MyError::CurrentDirectoryError(err))?
            .parent()
        {
            backup_file_path = dir.to_path_buf();
        } else {
            return Err(MyError::BackupDirectoryError);
        }
        let mut clone_file_path = backup_file_path.clone();
        backup_file_path.push(format!("{}-{}", subreddit, order));
        clone_file_path.push(format!("{}-{}-clone", subreddit, order));
        // println!("Dirs:{:?},{:?}", backup_file_path, clone_file_path);
        if !fs::metadata(&backup_file_path).is_ok() {
            fs::File::create(&backup_file_path).map_err(|err| MyError::CreateFileError(err))?;
        }
        Ok(Self {
            posts: HashSet::new(),
            url: url.to_string(),
            backup_file_path,
            clone_file_path,
        })
    }

    pub fn from_file(subreddit: &str, order: &str) -> Result<Self, MyError> {
        let mut data = Self::new(subreddit, order)?;
        let data_str: String;
        if fs::metadata(&data.clone_file_path).is_ok() {
            data_str = fs::read_to_string(&data.clone_file_path)
                .map_err(|err| MyError::FileReadError(data.clone_file_path.clone(), err))?;
            fs::copy(&data.clone_file_path, &data.backup_file_path)
                .map_err(|err| MyError::CopyFileError(err))?;
            fs::remove_file(&data.clone_file_path).map_err(|err| MyError::RemoveFileError(err))?;
        } else {
            data_str = fs::read_to_string(&data.backup_file_path)
                .map_err(|err| MyError::FileReadError(data.backup_file_path.clone(), err))?;
        }
        if let Ok(posts) = serde_json::from_str(&data_str) {
            data.posts = posts;
        }
        Ok(data)
    }

    pub fn save(&self) -> Result<(), MyError> {
        fs::copy(&self.backup_file_path, &self.clone_file_path)
            .map_err(|err| MyError::CopyFileError(err))?;
        let serialized_str = serde_json::to_string(&self.posts)?;
        fs::write(&self.backup_file_path, serialized_str)
            .map_err(|err| MyError::WriteFileError(err))?;
        // std::thread::sleep(std::time::Duration::from_secs(5));
        fs::remove_file(&self.clone_file_path).map_err(|err| MyError::RemoveFileError(err))?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), MyError> {
        let body: String = ureq::get(&self.url)
            .call()
            .map_err(|err| MyError::RequestError(self.url.clone(), err))?
            .into_string()
            .map_err(|err| MyError::StringConversionError(err))?;
        let res: Respone = serde_json::from_str(&body[0..])?;
        if res.data.children.is_empty() {
            return Err(MyError::InvalidSubredditError(self.url.clone()));
        }
        let current_time = Local::now().format("%H:%M:%S %d.%m.%Y");
        println!("Posts as of {}", current_time);
        res.data
            .children
            .into_iter()
            .map(|post| post.data)
            .for_each(|post| {
                if self.posts.insert(post.id.clone()) {
                    println!("{}", post);
                }
            });
        self.save()
    }
}
