use anyhow::Result;
use rusqlite::Connection;
use std::fs;

trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[&str]) -> Result<()>;
}

struct PingCommand {}

impl Command for PingCommand {
    fn get_name(&self) -> &'static str {
        "ping"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        println!("pong");
        Ok(())
    }
}

struct CountCommand {}

impl Command for CountCommand {
    fn get_name(&self) -> &'static str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        println!("counted {} args", args.len());
        Ok(())
    }
}

struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &'static str {
        "times"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        self.count += 1;
        println!("Times was called {} times", self.count);
        Ok(())
    }
}

struct MaxlenCommand {}

impl Command for MaxlenCommand {
    fn get_name(&self) -> &'static str {
        "maxlen"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        let mut max = 0;
        for arg in args {
            if arg.len() > max {
                max = arg.len()
            }
        }
        println!("The maximum length of an argument is {}", max);
        Ok(())
    }
}

struct Bookmark {
    name: String,
    url: String,
}
struct BmCommand {
    conn: Connection,
}

impl Command for BmCommand {
    fn get_name(&self) -> &'static str {
        "bm"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        if args[0] == "add" {
            if args.len() < 3 {
                anyhow::bail!("Not enough arguments for add! Synthax: bm add <name> <url>");
            }
            self.conn.execute(
                "insert into bookmarks (name,url) values (?1,?2);",
                (args[1], args[2]),
            )?;
        } else if args[0] == "search" {
            if args.len() < 2 {
                anyhow::bail!("Not enough arguments for add! Synthax: bm search <name>");
            }
            let mut stmt = self.conn.prepare("select * from bookmarks")?;
            let bookmarks = stmt.query_map([], |row| {
                Ok(Bookmark {
                    name: row.get("name")?,
                    url: row.get("url")?,
                })
            })?;
            for bookmark in bookmarks {
                let bookmark = bookmark?;
                if bookmark.name.contains(args[1]) {
                    println!("{} {}", bookmark.name, bookmark.url);
                }
            }
        } else {
            anyhow::bail!("Incorrect bm command! Use bm add or bm search")
        }
        Ok(())
    }
}

struct StopCommand {}

impl Command for StopCommand {
    fn get_name(&self) -> &'static str {
        "stop"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        println!("Execution stopped");
        Ok(())
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        let mut commands: Vec<Box<dyn Command>> = Vec::new();
        commands.push(Box::new(StopCommand {}));

        return Terminal { commands };
    }
    fn run(&mut self) -> Result<()> {
        let data = fs::read_to_string("src/input.txt")?;
        'main: for line in data.lines() {
            if line.trim().len() == 0 {
                continue;
            }
            let mut found = false;
            let mut args: Vec<&str> = Vec::new();
            for word in line.split_whitespace() {
                args.push(word);
            }
            for command in &mut self.commands {
                if command.get_name() == args[0] {
                    match command.exec(&args[1..]) {
                        Ok(_) => (),
                        Err(error) => println!("{error}"),
                    }

                    if command.get_name() == "stop" {
                        break 'main;
                    } else {
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                print!("Command \"{}\" not found! ", args[0]);
                for command in &mut self.commands {
                    if command.get_name().to_uppercase() == args[0].to_uppercase() {
                        print!("Did you mean \"{}\"?", command.get_name());
                        break;
                    }
                }
                print!("\n");
            }
        }
        Ok(())
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
}

fn main() -> Result<()> {
    let mut terminal = Terminal::new();

    let conn = Connection::open("bookmarks.db")?;
    let create = r"
create table if not exists bookmarks (
    name text    not null,
    url text not null
);
";
    conn.execute(create, ())?;

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(MaxlenCommand {}));
    terminal.register(Box::new(BmCommand { conn }));

    let _ = terminal.run();
    Ok(())
}
