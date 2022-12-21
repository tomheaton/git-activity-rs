use clap::Parser;
use std::{fs::OpenOptions, io::prelude::Write, path::Path, process::Command};
use chrono::{DateTime, Utc};

const FILE_NAME: &str = "ACTIVITY-TEST.md";
const FILE_TITLE: &str = "activity";

#[derive(Parser, Debug)]
struct Args {
    /// Number of commits to make.
    #[arg(default_value_t = 1)]
    commits: u8,

    /// Number of days to commit for.
    #[arg(default_value_t = 1)]
    days: u8,

    /// Start date for commits.
    #[arg(default_value_t = Utc::now().date_naive().to_string())]
    start_date: String
}

fn main() {
    println!("git-activity-rs");

    let args = Args::parse();
    println!("{:?}", args);

    // TODO: check current dir is a git repo.

    if !Path::new(FILE_NAME).exists() {
        println!("file does not exist!");

        let mut new_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(FILE_NAME)
            .unwrap();

        writeln!(new_file, "# {}\n", FILE_TITLE).unwrap();
        
        println!("file created!");
    } else {
        println!("file exists!");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_NAME)
        .unwrap();

    for count in 0..args.days {
        let now: DateTime<Utc> = Utc::now();

        // TODO: optimise the error handling.
        if let Err(e) = writeln!(file, "{} - {}\r", now, count) {
            eprintln!("Couldn't write to file: {}", e);
        }

        // TODO: git add and commit.
        let output = Command::new("git")
            .arg("add")
            .arg(FILE_NAME)
            .output()
            .expect("failed to execute process");

        println!("output: {:?}", output);
    }
}
