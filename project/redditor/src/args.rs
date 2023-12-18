use clap::{Parser, ValueEnum};
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Order {
    New,
    Hot,
    Top,
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Execcutable that shows new posts from a subreddit after a set ammount of time has passed"
)]

pub struct Args {
    ///The subreddit the data will be provided for
    #[arg(short, long)]
    subreddit: String,

    ///The order the posts are listed in(defaults to "hot")
    #[arg(short, long, value_enum,default_value_t = Order::Hot)]
    order: Order,

    ///Only show posts different from the ones printed after the last execution of the command(subreddit and order specific)
    #[arg(short, long)]
    previous: bool,

    ///The ammount of seconds between updates(defaults to 60 seconds)
    #[arg(short, long, default_value_t = 60)]
    time: u64,
}

impl Args {
    pub fn parsed() -> Self {
        Self::parse()
    }
    pub fn get_subreddit(&self) -> &str {
        &self.subreddit
    }
    pub fn get_order(&self) -> &str {
        match &self.order {
            Order::Hot => "hot",
            Order::New => "new",
            Order::Top => "top",
        }
    }
    pub fn get_previous(&self) -> bool {
        self.previous
    }
    pub fn get_time(&self) -> Duration {
        Duration::from_secs(self.time)
    }
}
