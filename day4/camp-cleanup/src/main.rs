use camp_cleanup::SectionAssignment;
use std::fs;

fn main() {
    let input_data = fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut assignment_groups = Vec::new();
    for line in input_data.lines() {
        assignment_groups.push(SectionAssignment::pair_from_line(line));
    }

    let mut full_overlap = 0;
    let mut partial_overlap = 0;
    for pair in assignment_groups {
        if SectionAssignment::has_full_overlap(&pair) {
            full_overlap += 1;
        }
        if SectionAssignment::has_partial_overlap(&pair) {
            partial_overlap += 1;
        }
    }

    println!("Full overlap: {}", full_overlap);
    println!("Partial overlap: {}", partial_overlap);
}
