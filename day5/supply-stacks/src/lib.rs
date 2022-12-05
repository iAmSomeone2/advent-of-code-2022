use regex::Regex;
use substring::{self, Substring};

#[derive(Debug)]
pub struct MoveInstruction {
    /// Number of crates to move
    count: usize,
    /// Index of stack to grap crates from
    src_idx: usize,
    /// Index of stack to move crates to
    target_idx: usize,
}

impl MoveInstruction {
    const PATTERN: &str = "^move (?P<count>\\d+) from (?P<src>\\d+) to (?P<target>\\d+)$";

    /// Create a MoveInstruction instance from a given string.
    pub fn from_string(s: &str) -> Self {
        let instruction_regex = Regex::new(MoveInstruction::PATTERN).unwrap();
        let captures = instruction_regex.captures(s).unwrap();

        let count = match captures.name("count") {
            Some(cap) => cap.as_str().parse().unwrap(),
            None => 0,
        };

        let src_idx = match captures.name("src") {
            Some(cap) => cap.as_str().parse().unwrap(),
            None => 0,
        };

        let target_idx = match captures.name("target") {
            Some(cap) => cap.as_str().parse().unwrap(),
            None => 0,
        };

        MoveInstruction {
            count,
            src_idx,
            target_idx,
        }
    }

    /// Execute a move instruction on a set of Stacks
    pub fn execute(&self, stacks: &mut Vec<Stack>, print_step: bool) {
        // Repeat 'count' amount of times
        for _ in 0..self.count {
            // Pop a crate off of the source stack
            let pop_crate = stacks[self.src_idx - 1].crates.pop().unwrap();
            // Push it onto the target stack
            stacks[self.target_idx - 1].crates.push(pop_crate);
        }

        if print_step {
            Stack::print(stacks);
        }
    }
}

#[derive(Debug)]
pub struct CargoCrate {
    pub label: char,
}

impl CargoCrate {
    /// Create a new Crate instance from a 3-character string
    ///
    /// Example: \[Q\]
    pub fn from_string_optional(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        let label = s.chars().collect::<Vec<char>>()[1];

        Some(CargoCrate { label })
    }
}

#[derive(Debug)]
pub struct Stack {
    index: usize,
    pub crates: Vec<CargoCrate>,
}

impl Stack {
    pub fn from_column(col: &str) -> Self {
        let lines: Vec<&str> = col.lines().collect();

        let mut crates = Vec::new();
        let mut index = 0;
        // Read column from bottom to top
        for i in (0..lines.len()).rev() {
            if i == lines.len() - 1 {
                // Get the stack index
                let line = lines[i].trim();
                index = line.parse().unwrap();
            } else {
                // Add crates
                match CargoCrate::from_string_optional(&lines[i]) {
                    Some(c) => {
                        crates.push(c);
                    }
                    None => {}
                }
            }
        }

        Stack { index, crates }
    }

    fn from_index_line(index_line: &str) -> Vec<Self> {
        let mut stacks = Vec::new();
        for index_str in index_line.split_whitespace() {
            let index = index_str.trim().parse().unwrap();
            stacks.push(Self {
                index,
                crates: Vec::new(),
            });
        }

        stacks
    }

    pub fn from_lines(lines: &Vec<&str>) -> Vec<Self> {
        // Create stacks from indicies from last line
        let index_line = lines[lines.len() - 1];
        let mut stacks = Stack::from_index_line(index_line);

        let col_width = 4;
        for i in (0..(lines.len() - 1)).rev() {
            let line = lines[i];
            for j in (0..line.len()).step_by(col_width) {
                let stack_idx = j / col_width;
                let chunk = line.substring(j, j + col_width).trim();

                match CargoCrate::from_string_optional(chunk) {
                    Some(c) => stacks[stack_idx].crates.push(c),
                    None => {}
                }
            }
        }

        stacks
    }

    pub fn print(stacks: &Vec<Stack>) {
        // Get tallest stack
        let mut max_height = stacks[0].crates.len();
        for i in 1..stacks.len() {
            let stack = &stacks[i];
            if stack.crates.len() > max_height {
                max_height = stack.crates.len();
            }
        }

        // Start printing from the top of the tallest stack
        let mut stack_str = String::new();
        for i in (0..max_height).rev() {
            for stack in stacks {
                let mut crate_label = String::from("   ");
                if stack.crates.len() >= (i + 1) {
                    crate_label = String::new();
                    crate_label.push('[');
                    crate_label.push(stack.crates[i].label);
                    crate_label.push(']');
                }
                stack_str.push_str(crate_label.as_str());
                stack_str.push(' ');
            }
            stack_str.push('\n');
        }

        for stack in stacks {
            let mut index_label = String::from(" ");
            index_label.push_str(stack.index.to_string().as_str());
            index_label.push_str("  ");
            stack_str.push_str(index_label.as_str());
        }

        println!("{stack_str}\n");
    }
}

#[cfg(test)]
mod test {
    mod move_instruction {
        use crate::MoveInstruction;

        #[test]
        fn create_from_string() {
            let instruction_str = "move 35 from 3 to 2";
            let move_instruction = MoveInstruction::from_string(instruction_str);

            assert_eq!(move_instruction.count, 35);
            assert_eq!(move_instruction.src_idx, 3);
            assert_eq!(move_instruction.target_idx, 2);
        }
    }

    mod stack {
        use crate::Stack;

        #[test]
        fn create_from_column() {
            let input_data = "\
[N]
[Z]
 1 ";

            let mut stack = Stack::from_column(&input_data);

            assert_eq!(1, stack.index, "Stack index should be {}", 1);
            assert_eq!(2, stack.crates.len(), "Stack should contain {} crates", 2);

            let top_crate_label = match stack.crates.pop() {
                Some(c) => c.label,
                None => {
                    panic!("1st crate missing from stack");
                }
            };
            assert_eq!('N', top_crate_label, "Label of top crate should be {}", 'N');

            let bottom_crate_label = match stack.crates.pop() {
                Some(c) => c.label,
                None => {
                    panic!("2nd Crate missing from stack");
                }
            };
            assert_eq!(
                'Z', bottom_crate_label,
                "Label of bottom crate should be {}",
                'Z'
            );
        }

        #[test]
        fn create_from_index_line() {
            let input_data = " 1   2   3   4   5   6   7   8   9 ";

            let stacks = Stack::from_index_line(input_data);

            assert_eq!(stacks.len(), 9, "{} Stack instances should be created", 9);
        }
    }
}
