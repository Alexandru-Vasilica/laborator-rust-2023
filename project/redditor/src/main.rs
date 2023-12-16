mod Reddit;
use anyhow::Result;

fn main() -> Result<()> {
    let mut data =
        Reddit::SubredditUpdate::new("https://www.reddit.com/r/leagueoflegends/new.json");
    data.update()?;
    Ok(())
}
