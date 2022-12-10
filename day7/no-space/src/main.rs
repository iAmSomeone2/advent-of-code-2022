use no_space::NsFile;
use regex::Regex;
use std::fs;

const CHANGE_DIR_PATTERN: &'static str = "^\\$ cd (?P<dir>/|\\w+|\\.{2})$";

fn main() {
    let cd_matcher = Regex::new(CHANGE_DIR_PATTERN).unwrap();
    let input_data = fs::read_to_string("test_input.txt").expect("Failed to read input file");

    let mut root_dir = NsFile::new_dir("/");
    let mut current_dir = &mut root_dir;
    let mut parent_dirs: Vec<&NsFile> = vec![];
    let mut is_listing = false; // 'true' when "$ ls" is detected until another command is found
    for line in input_data.lines() {
        if is_listing {
            if line.starts_with('$') {
                is_listing = false;
                continue;
            }

            // Create new file and add it as a child to the current directory
            let new_file = NsFile::from_line(line);
            current_dir.add_child(new_file);
        }

        // The next group of lines will be directory contents
        if line == "$ ls" {
            is_listing = true;
            continue;
        }

        // The current directory is being changed
        if line.starts_with("$ cd") {
            let caps = cd_matcher.captures(line).unwrap();
            let name = caps.name("name").unwrap().as_str();

            if name != ".." {
                // Change into a child directory
                match current_dir.get_child(name) {
                    Some(dir) => {
                        current_dir = &mut dir;
                    }
                    None => {}
                }
            }
        }
    }
}
