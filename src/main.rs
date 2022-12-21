use clap::Parser;
use std::{fs::OpenOptions, io::prelude::Write, path::Path, process::Command};
use chrono::{Utc, Duration};

const FILE_NAME: &str = "ACTIVITY-TEST.md";
const FILE_TITLE: &str = "activity";

#[derive(Parser, Debug)]
struct Args {
    /// Number of commits to make.
    #[arg(default_value_t = 1, short, long)]
    commits: i16,

    /// Number of days to commit for.
    #[arg(default_value_t = 1, short, long)]
    days: i16,

    /// Start date for commits.
    #[arg(default_value_t = Utc::now().date_naive().to_string())]
    start_date: String
}

fn main() {
    println!("git-activity-rs");

    let args = Args::parse();
    println!("{:?}", args);

    // TODO: better error handling.
    if args.commits < 1 {
        println!("commits must be greater than 0");
        return;
    }
    if args.days < 1 {
        println!("days must be greater than 0");
        return;
    }

    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute process");

    if !output.status.success() {
        println!("git repo not found!");
    
        let output = Command::new("git")
        .arg("init")
        .output()
        .expect("failed to execute process");

        println!("git repo created! (output: {:?})", output);
    } else {
        println!("git repo found!");
    }

    if !Path::new(FILE_NAME).exists() {
        println!("file does not exist!");

        let mut new_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(FILE_NAME)
            .unwrap();

        if let Err(e) = writeln!(new_file, "# {}\n", FILE_TITLE) {
            eprintln!("Couldn't write to file: {}", e);
        }
        
        println!("file created!");
    } else {
        println!("file exists!");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_NAME)
        .unwrap();

    let commits_per_day = args.commits / args.days;
    let mut commit_count = 0;

    let now = Utc::now();
    for count in (0..args.days).rev(){
        let date = now - Duration::days(count as i64);
        println!("date: {}", date.date_naive());

        for _ in 0..commits_per_day {
            // TODO: optimise the error handling.
            if let Err(e) = writeln!(file, "{} - {}\r", date, commit_count) {
                eprintln!("Couldn't write to file: {}", e);
            }

            // let message = format!("{} - {}", date, count);
            // add_and_commit(message);
            commit_count += 1;
        }
    }

    if let Err(e) = writeln!(file, "\n") {
        eprintln!("Couldn't write to file: {}", e);
    }
    // add_and_commit("save".to_string());
}


fn _add_and_commit(commit_message: String) {
    Command::new("git")
        .arg("add")
        .arg(FILE_NAME)
        .output()
        .expect("failed to execute process");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output()
        .expect("failed to execute process");
}
