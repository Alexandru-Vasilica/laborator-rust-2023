mod args;
mod errors;
mod reddit;

use reddit::SubredditUpdate;

fn main() {
    let args = args::Args::parsed();
    // println!("{:?}", args);

    let data_result = match args.get_previous() {
        true => SubredditUpdate::from_file(args.get_subreddit(), args.get_order()),
        false => SubredditUpdate::new(args.get_subreddit(), args.get_order()),
    };
    if let Ok(mut data) = data_result {
        loop {
            if let Err(e) = data.update() {
                eprint!("Error:{}", e);
                break;
            }
            std::thread::sleep(args.get_time());
        }
    } else {
        eprintln!("Error:{}", data_result.unwrap_err());
    }
}
