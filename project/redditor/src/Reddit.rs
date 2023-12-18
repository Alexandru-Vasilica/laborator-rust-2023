use anyhow::Result;
use chrono::Local;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;

use std::fmt::Display;
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};

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

pub struct SubredditUpdate {
    posts: HashSet<String>,
    url: String,
    backup_file_path: PathBuf,
    clone_file_path: PathBuf,
}
impl SubredditUpdate {
    pub fn new(subreddit: &str, order: &str) -> Result<Self> {
        let url = format!("https://www.reddit.com/r/{subreddit}/{order}.json");
        let mut backup_file_path = env::current_dir()?;
        let mut clone_file_path = backup_file_path.clone();
        backup_file_path.push(format!("{}-{}", subreddit, order));
        clone_file_path.push(format!("{}-{}-clone", subreddit, order));
        // print!("Dirs:{:?},{:?}", backup_file_path, clone_file_path);
        if !fs::metadata(&backup_file_path).is_ok() {
            fs::File::create(&backup_file_path)?;
        }
        Ok(Self {
            posts: HashSet::new(),
            url: url.to_string(),
            backup_file_path,
            clone_file_path,
        })
    }
    pub fn from_file(subreddit: &str, order: &str) -> Result<Self> {
        let mut data = Self::new(subreddit, order)?;
        let data_str: String;
        if fs::metadata(&data.clone_file_path).is_ok() {
            data_str = fs::read_to_string(&data.clone_file_path)?;
            fs::copy(&data.clone_file_path, &data.backup_file_path)?;
            fs::remove_file(&data.clone_file_path)?;
        } else {
            data_str = fs::read_to_string(&data.backup_file_path)?;
        }
        if let Ok(posts) = serde_json::from_str(&data_str) {
            data.posts = posts;
        }
        Ok(data)
    }
    pub fn save(&self) -> Result<()> {
        fs::copy(&self.backup_file_path, &self.clone_file_path)?;
        let serialized_str = serde_json::to_string(&self.posts)?;
        fs::write(&self.backup_file_path, serialized_str)?;
        std::thread::sleep(std::time::Duration::from_secs(5));
        fs::remove_file(&self.clone_file_path)?;
        Ok(())
    }
    pub fn update(&mut self) -> Result<()> {
        let body: String = ureq::get(&self.url).call()?.into_string()?;
        let res: Respone = serde_json::from_str(&body[0..])?;
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
        self.save()?;
        Ok(())
    }
}
