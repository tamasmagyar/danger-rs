use clap::{Arg, Command};
use danger_rs::run_danger;

fn main() {
    let matches = Command::new("danger-rs")
        .version("0.2.0")
        .about("Automate your code review process with Rust")
        .arg(
            Arg::new("dangerfile")
                .short('d')
                .long("dangerfile")
                .value_name("FILE")
                .help("Path to Dangerfile"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output"),
        )
        .get_matches();

    if let Err(e) = run_danger() {
        eprintln!("Error running danger: {}", e);
        std::process::exit(1);
    }
}
