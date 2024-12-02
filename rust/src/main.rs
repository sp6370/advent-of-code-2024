use std::env;
use std::process;

mod solutions {
    // Import day modules
    pub mod day01;
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a day is specified
    if args.len() < 2 {
        eprintln!("Please specify a day (e.g., cargo run -- 1)");
        process::exit(1);
    }

    // Parse the day number
    let day: u32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid day number");
            process::exit(1);
        }
    };

    // Run the appropriate day's solution
    match day {
        1 => {
            if let Err(e) = solutions::day01::solve() {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        },
        // Add more days as you implement them
        _ => {
            eprintln!("Day {} not implemented", day);
            process::exit(1);
        }
    }
}