mod Reddit;
use anyhow::Result;

fn main() -> Result<()> {
    let mut data = Reddit::SubredditUpdate::new("leagueoflegends", "hot");
    data.update()?;
    Ok(())
}
