use part1::RpsRound;
use std::fs;

fn main() {
    let input_data = fs::read_to_string("full_input.txt").expect("Failed to read input file");

    let rps_rounds = RpsRound::from_lines_2(input_data.lines());

    println!("Imported {} rounds", rps_rounds.len());

    let total_score = RpsRound::total_score(rps_rounds);
    println!("Total score: {total_score}");
}
