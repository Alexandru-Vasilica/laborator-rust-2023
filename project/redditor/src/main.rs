mod Reddit;
mod args;

use anyhow::Result;
use std::time::Duration;
use Reddit::SubredditUpdate;

fn main() -> Result<()> {
    let args = args::Args::parsed();
    // println!("{:?}", args);

    let mut data: SubredditUpdate;
    match args.get_previous() {
        true => data = SubredditUpdate::from_file(args.get_subreddit(), args.get_order())?,
        false => data = SubredditUpdate::new(args.get_subreddit(), args.get_order())?,
    }
    loop {
        data.update()?;
        std::thread::sleep(Duration::from_secs(20));
    }
}
