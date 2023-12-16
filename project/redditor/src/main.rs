mod Reddit;
mod args;

use anyhow::Result;
use chrono::Local;
use std::time::Duration;

fn main() -> Result<()> {
    let args = args::Args::parsed();
    // println!("{:?}", args);

    let mut data = Reddit::SubredditUpdate::new(args.get_subreddit(), args.get_order());
    loop {
        let current_time = Local::now().format("%H:%M:%S %d.%m.%Y");
        println!("Posts as of {}", current_time);
        data.update()?;
        std::thread::sleep(Duration::from_secs(20));
    }
}
