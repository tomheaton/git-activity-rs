use clap::Parser;
use std::{fs::OpenOptions, io::prelude::*, path::Path, time::Duration, num::ParseIntError};
use chrono::{DateTime, Utc};

#[derive(Parser, Debug)]
struct Args {
    /// Number of commits to make.
    #[arg(default_value_t = 1)]
    commits: u8,

    /// Number of days to commit for.
    #[arg(default_value_t = 1)]
    days: u8,

    // /// Start date for commits.
    // #[arg(value_parser = parse_duration, default_value_t = Utc::now().timestamp().parse_duration)]
    // start_date: Duration,
}

fn main() {
    println!("git-activity-rs");

    let args = Args::parse();

    println!("{:?}", args);

    // let output = Command::new("git")
    //     .arg("log")
    //     .output()
    //     .expect("failed to execute process");
    // println!("output: {:?}", output.stdout);

    edit_file(args);
}

fn edit_file(args: Args) {
    let file_exists = Path::new("HISTORY.md").exists();

    if !file_exists {
        println!("file does not exist");
        let mut new_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("ACTIVITY.md")
            .unwrap();

        writeln!(new_file, "# history\n").unwrap();
    }
    println!("file exists");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("ACTIVITY.md")
        .unwrap();

    for count in 0..args.days {
        let now: DateTime<Utc> = Utc::now();

        if let Err(e) = writeln!(file, "{} - {}\r", now, count) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}


// TODO: use with start_date arg.
fn _parse_duration(arg: &str) -> Result<Duration, ParseIntError> {
    let seconds = arg.parse()?;
    return Ok(std::time::Duration::from_secs(seconds));
}
