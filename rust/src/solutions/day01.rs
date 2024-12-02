use anyhow::Result;
use std::path::Path;

pub fn solve() -> Result<()> {
    let input_path = Path::new("inputs/day01.txt");

    println!("Day 1 Solutions: {}", input_path.display());
    Ok(())
}
