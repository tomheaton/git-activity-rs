use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
struct Args {
    name: String,
    days: u8,
}

fn main() {
    println!("git-activity-rs");

    let args = Args::parse();

    println!("{:?}", args);

    // TODO: use git2-rs to get the git log
    let output = Command::new("git")
        .arg("log")
        .arg("--author")
        .arg(args.name)
        .arg("--since")
        .arg(format!("{} days ago", args.days))
        .output()
        .expect("failed to execute process");

        println!("output: {:?}", output);
        println!("status: {}", output.status);
}
