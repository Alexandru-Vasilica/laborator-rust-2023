mod Reddit;
use anyhow::Result;

fn main() -> Result<()> {
    let mut data = Reddit::SubredditUpdate::new("cats", "hot");
    data.update()?;
    Ok(())
}
