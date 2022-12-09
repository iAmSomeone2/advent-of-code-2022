use no_space::NsFile;
use regex::Regex;
use std::fs;

const CHANGE_DIR_PATTERN: &'static str = "^\\$ cd (?P<dir>/|\\w+|\\.{2})$";

fn main() {
    let input_data = fs::read_to_string("test_input.txt").expect("Failed to read input file");
}
