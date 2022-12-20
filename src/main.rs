use clap::Parser;
use std::{process::Command, fs::OpenOptions, io::prelude::*, path::Path};
use chrono::{DateTime, Utc};

#[derive(Parser, Debug)]
struct Args {
    name: String,
    count: u8,
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

    for _ in 0..args.count {
        let now: DateTime<Utc> = Utc::now();

        if let Err(e) = writeln!(file, "{} - {}\r", now, args.name) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
