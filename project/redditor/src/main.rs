mod Reddit;
use anyhow::Result;
use chrono::Local;
use std::time::Duration;

fn main() -> Result<()> {
    let mut data = Reddit::SubredditUpdate::new("funny", "new");
    loop {
        let current_time = Local::now().format("%H:%M:%S %d.%m.%Y");
        println!("Posts as of {}", current_time);
        data.update()?;
        std::thread::sleep(Duration::from_secs(20));
    }
}
