use anyhow::Result;
use serde_derive::Deserialize;
use std::collections::HashSet;

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

pub struct SubredditUpdate {
    posts: HashSet<String>,
    url: String,
}
impl SubredditUpdate {
    pub fn new(subreddit: &str, order: &str) -> Self {
        let mut url = format!("https://www.reddit.com/r/{subreddit}/{order}.json");
        Self {
            posts: HashSet::new(),
            url: url.to_string(),
        }
    }
    pub fn update(&mut self) -> Result<()> {
        let body: String = ureq::get(&self.url).call()?.into_string()?;
        let res: Respone = serde_json::from_str(&body[0..])?;
        res.data
            .children
            .into_iter()
            .map(|post| post.data)
            .for_each(|post| {
                if self.posts.insert(post.id.clone()) {
                    println!("{:?}", post);
                }
            });
        Ok(())
    }
}
