use anyhow::Result;
use serde_derive::Deserialize;

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
    posts: Vec<PostData>,
    url: String,
}
impl SubredditUpdate {
    pub fn new(url: &str) -> Self {
        Self {
            posts: Vec::new(),
            url: url.to_string(),
        }
    }
    pub fn update(&mut self) -> Result<()> {
        let body: String = ureq::get(&self.url).call()?.into_string()?;
        let res: Respone = serde_json::from_str(&body[0..])?;
        self.posts = res
            .data
            .children
            .into_iter()
            .map(|post| post.data)
            .collect();
        for post in &self.posts {
            println!("{:?}", post);
        }
        Ok(())
    }
}
