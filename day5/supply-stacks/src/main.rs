use std::env;
use std::fs;
use supply_stacks::{MoveInstruction, Stack};

/// Split input data into stack data and instructions
fn split_input(input_data: &String) -> (Vec<&str>, Vec<&str>) {
    let mut has_split = false;
    let mut stack_data = vec![];
    let mut instruction_data = vec![];

    for line in input_data.lines() {
        if has_split {
            instruction_data.push(line);
            continue;
        }

        if line.is_empty() {
            has_split = true;
            continue;
        }

        stack_data.push(line);
    }

    (stack_data, instruction_data)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    

    let input_data = fs::read_to_string("input.txt").expect("Failed to read input file");

    let input_data = split_input(&input_data);

    let mut stacks = Stack::from_lines(&input_data.0);
    for line in input_data.1 {
        let move_instruction = MoveInstruction::from_string(line);
        move_instruction.execute_v2(&mut stacks, true);
    }

    let mut tops = String::new();
    for stack in stacks {
        tops.push(stack.crates[stack.crates.len() - 1].label);
    }

    println!("Result: {}", tops);
}
