use std::fs;
use clap::Parser;
use serde_json::Value;
use diffR::{arg_parsing::Arguments, diff::delta};

fn main() {
    let args = Arguments::parse();

    let baseline_json_file = fs::read_to_string(&args.baseline_file_path).expect("Unable to read baseline json file!");
    let other_json_file = fs::read_to_string(&args.comparee_file_path).expect("Unable to read other json file!");
    

    let baseline_json: Value = serde_json::from_str(&baseline_json_file).expect("Invalid JSON in specified baseline file!");
    let json2: Value = serde_json::from_str(&other_json_file).expect("Invalid JSON in json file to compare!");

    // Compute the differences
    let differences = delta::get_difference_json(&baseline_json, &json2);

    // Print the differences
    println!("{}", serde_json::to_string_pretty(&differences).unwrap());
}

// todo: add old-new output in json
// todo: add local watch mode either in filesystem or via http

