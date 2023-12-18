pub use clap::{Parser, ValueEnum};
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

    ///The order the posts are listed in "hot" order
    #[arg(short, long, value_enum,default_value_t = Order::Hot)]
    order: Order,

    ///Only show posts different from the ones printed after the last execution of the command(subreddit specific)
    #[arg(short, long)]
    previous: bool,
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
            Order::Hot => return "hot",
            Order::New => return "new",
            Order::Top => return "top",
        }
    }
    pub fn get_previous(&self) -> bool {
        self.previous
    }
}
