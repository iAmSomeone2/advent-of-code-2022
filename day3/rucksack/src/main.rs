use rucksack::Rucksack;
use std::fs;

fn main() {
    let input_data = fs::read_to_string("input.txt").expect("Failed to read input file");
    let rucksacks = Rucksack::groups_from_lines(&input_data.lines().collect());

    println!("Rucksack count: {}", rucksacks.len());
    let mut total_priority: u64 = 0;
    for group in rucksacks {
        total_priority += u64::from(Rucksack::get_group_overlap_priority(&group));
    }

    println!("Total priority: \'{}\'", total_priority);
}
