mod Reddit;
mod args;
mod errors;

use std::time::Duration;
use Reddit::SubredditUpdate;

fn main() {
    let args = args::Args::parsed();
    // println!("{:?}", args);

    let data_result;
    match args.get_previous() {
        true => data_result = SubredditUpdate::from_file(args.get_subreddit(), args.get_order()),
        false => data_result = SubredditUpdate::new(args.get_subreddit(), args.get_order()),
    }
    if let Ok(mut data) = data_result {
        loop {
            match data.update() {
                Err(error) => {
                    eprintln!("Error:{}", error);
                    break;
                }
                Ok(()) => {}
            }
            std::thread::sleep(Duration::from_secs(20));
        }
    } else {
        eprintln!("Error:{}", data_result.unwrap_err());
    }
    return;
}
