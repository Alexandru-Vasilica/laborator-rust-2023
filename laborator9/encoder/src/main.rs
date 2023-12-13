use base64;
use clap::Parser;
use std::fs;
use std::io;
use std::io::BufRead;

mod os;
mod test;

#[derive(Parser, Debug)]
#[command(version, about = "Execcutable that encodes the given content")]

struct Args {
    ///Sets an input file(if not provided stdin will be used)
    #[arg(long)]
    input: Option<String>,

    ///Sets an output file(if not provided stdout will be used)
    #[arg(long)]
    output: Option<String>,
}

fn main() {
    println!(
        "-----encoder version {} built for {}-----",
        env!("CARGO_PKG_VERSION"),
        os::get_os_name()
    );
    let args = Args::parse();
    println!("{:?}", args);
    let input;
    if let Some(file_in) = args.input {
        input = fs::read(file_in)
            .expect("Could not read from input file. Make sure the path is correct");
    } else {
        println!("Enter your string:");
        let mut str = String::new();
        io::stdin()
            .lock()
            .read_line(&mut str)
            .expect("Couldn't read input");
        input = str.into_bytes();
    }
    let output = base64::encode(&input);
    if let Some(file_out) = args.output {
        fs::write(file_out, output)
            .expect("Could not write to output file. Make sure the path is correct");
        println!("Encoding complete");
    } else {
        println!("Encoding result:");
        println!("{}", output);
    }
}
