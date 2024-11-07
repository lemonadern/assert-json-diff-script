use serde_json::Value;
use assert_json_diff::assert_json_eq;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file1.json> <file2.json>", args[0]);
        std::process::exit(1);
    }

    let file1_content = fs::read_to_string(&args[1])?;
    let file2_content = fs::read_to_string(&args[2])?;

    let json1: Value = serde_json::from_str(&file1_content)?;
    let json2: Value = serde_json::from_str(&file2_content)?;

    assert_json_eq!(json1, json2);
    println!("The JSON files are equivalent.");

    Ok(())
}
