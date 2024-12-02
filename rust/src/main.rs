use std::fs::read_to_string;

fn day01() {
    println!("Current working directory: {}", std::env::current_dir().unwrap().display());
    
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    for line in read_to_string("src/inputs/day01.txt").unwrap().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(val_a), Ok(val_b)) = (
                parts[0].parse(), parts[1].parse()
            ) {
                list_a.push(val_a);
                list_b.push(val_b);
            } else {
                eprintln!("Error parsing line: {}", line);
            }
        }
    }
}

fn main() {
    day01();
}