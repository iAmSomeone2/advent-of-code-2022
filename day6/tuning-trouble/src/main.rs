use std::fs;
use tuning_trouble::{get_marker_range, get_start_of_message_range};

fn main() {
    let input_data = fs::read_to_string("input.txt").expect("Failed to read input file");

    let marker_range = get_marker_range(input_data.as_str()).unwrap_or(0..0);
    println!("Marker range: {:?}", marker_range);
    let start_of_message_range = get_start_of_message_range(input_data.as_str()).unwrap_or(0..0);
    println!("Start of message range: {:?}", start_of_message_range);
}
