use anyhow::Result;
use std::fs;

trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: &[&str]);
}

struct PingCommand {}

impl Command for PingCommand {
    fn get_name(&self) -> &'static str {
        "ping"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("pong");
    }
}

struct CountCommand {}

impl Command for CountCommand {
    fn get_name(&self) -> &'static str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("counted {} args", args.len());
    }
}

struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &'static str {
        "times"
    }

    fn exec(&mut self, args: &[&str]) {
        self.count += 1;
        println!("Times was called {}", self.count);
    }
}

struct LongestWordCommand {}

struct StopCommand {}

impl Command for StopCommand {
    fn get_name(&self) -> &'static str {
        "stop"
    }

    fn exec(&mut self, args: &[&str]) {
        println!("Execution stopped");
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
                    command.exec(&args[1..]);

                    if command.get_name() == "stop" {
                        break 'main;
                    } else {
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                println!("Command {} not found!", args[0]);
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

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run();
    Ok(())
}
