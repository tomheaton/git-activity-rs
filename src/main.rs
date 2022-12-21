use clap::Parser;
use std::{fs::OpenOptions, io::prelude::Write, path::Path, process::Command};
use chrono::{Utc, Duration, NaiveDate, NaiveDateTime, DateTime};

const FILE_NAME: &str = "ACTIVITY-TEST.md";
const FILE_TITLE: &str = "activity";

/// Args for the program.
#[derive(Parser, Debug)]
struct Args {
    /// Number of commits to make.
    #[arg(short, long, default_value_t = 1)]
    commits: i16,

    /// Number of days to commit for.
    #[arg(short, long, default_value_t = 1)]
    days: i16,

    /// Start date for commits.
    #[arg(short, long, default_value_t = Utc::now().date_naive().to_string())]
    start: String
}

fn main() {
    println!("git-activity-rs");

    let args = Args::parse();
    println!("{:?}", args);

    // TODO: add better error handling.
    if args.commits < 1 {
        println!("commits must be greater than 0!");
        return;
    }
    if args.days < 1 {
        println!("days must be greater than 0!");
        return;
    }
    if args.days > args.commits {
        println!("days must be greater than or equal to commits!");
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
    println!("commits/day: {}", commits_per_day);
    let mut count = 0;

    let naive_date = NaiveDate::parse_from_str(&args.start, "%Y-%m-%d").unwrap();
    #[allow(deprecated)]
    let naive_datetime: NaiveDateTime = naive_date.and_hms(0,0,0);
    let start = DateTime::<Utc>::from_utc(naive_datetime, Utc);
    
    println!("start: {}", start);

    for day in (0..args.days).rev(){
        let date = start - Duration::days(day as i64);
        println!("date: {}", date.date_naive());

        for _ in 0..commits_per_day {
            // TODO: optimise the error handling.
            if let Err(e) = writeln!(file, "{} - {}\r", date, count) {
                eprintln!("Couldn't write to file: {}", e);
            }

            let message = format!("{} - {}", date, count);
            add_and_commit(message, date);
            count += 1;
        }
    }

    // if let Err(e) = writeln!(file) {
    //     eprintln!("Couldn't write to file: {}", e);
    // }
    // add_and_commit("save".to_string(), Utc::now());
}


fn add_and_commit(commit_message: String, date: DateTime<Utc>) {
    let output_add = Command::new("git")
        .arg("add")
        .arg(FILE_NAME)
        .output()
        .expect("failed to execute process");

    let output_commit = Command::new("git")
        .env("GIT_COMMITTER_DATE", format!("{}", date))
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .arg("--date")
        .arg(format!("{}", date))
        .output()
        .expect("failed to execute process");

    println!("add: {:?}", output_add);
    println!("commit: {:?}", output_commit);
}
