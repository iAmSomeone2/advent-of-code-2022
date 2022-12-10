use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use regex::Regex;

const FILE_LISTING_PATTERN: &'static str = "^(?P<size>\\d+) (?P<name>\\w+\\.?(\\w+)?)";
const DIR_LISTING_PATTERN: &'static str = "^dir (?P<name>\\w+)";

/// Basic 'File' implementation for the puzzle
pub struct NsFile<'a, 'b: 'a> {
    /// Name of the file or directory
    name: &'a str,
    /// Size (in bytes) of the NsFile. Will be 0 if the instance is a directory
    size: usize,
    /// Flag specifying if this instance is a directory and can have children
    is_dir: bool,

    /// This file's parent directory
    parent: Option<&'b NsFile<'a, 'b>>,

    /// Child NsFile instances
    children: HashMap<&'a str, NsFile<'a, 'b>>,
}

impl Display for NsFile<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        return if self.is_dir {
            write!(f, "{} (dir)", self.name)
        } else {
            write!(f, "{} (file, size={})", self.name, self.size)
        };
    }
}

impl<'a, 'b> NsFile<'a, 'b> {
    /// Create a new named file
    pub fn new_file(name: &'a str, size: usize) -> Self {
        NsFile {
            name,
            size,
            is_dir: false,
            parent: None,
            children: HashMap::new(),
        }
    }

    /// Create a new named directory
    pub fn new_dir(name: &'a str) -> Self {
        NsFile {
            name,
            size: 0,
            is_dir: true,
            parent: None,
            children: HashMap::new(),
        }
    }

    pub fn from_line(line: &'a str) -> Self {
        let file_matcher = Regex::new(FILE_LISTING_PATTERN).unwrap();
        let dir_matcher = Regex::new(DIR_LISTING_PATTERN).unwrap();

        match file_matcher.captures(line) {
            Some(caps) => {
                // Line describes a file
                let size = caps.name("size").unwrap().as_str();
                let size = size.parse().unwrap();
                let name = caps.name("name").unwrap().as_str();

                NsFile::new_file(name, size)
            }
            None => {
                // Line describes a dir
                let caps = dir_matcher.captures(line).unwrap();
                let name = caps.name("name").unwrap().as_str();

                NsFile::new_dir(name)
            }
        }
    }

    pub fn set_parent(&mut self, parent: &'b NsFile<'a, 'b>) {
        self.parent = Some(parent);
    }

    pub fn add_child(&mut self, child: NsFile<'a, 'b>) {
        if !self.is_dir {
            // Children can only be added to directories
            return;
        }

        // self.children.push(child);
        self.children.insert(child.name, child);
    }

    pub fn get_child(&self, name: &str) -> Option<&mut NsFile> {
        // self.children.get(name)
        return match self.children.get(name) {
            Some(mut child) => Some(&mut child),
            None => None,
        };
    }

    pub fn total_size(&self) -> usize {
        if !self.is_dir {
            return self.size;
        }

        // Calculate the total sizes of all child files
        0
    }

    pub fn get_layout_string(&self, level: Option<u32>) -> String {
        todo!();

        let level = level.unwrap_or(1);
        let mut left_pad = String::new();
        for _ in 0..level {
            left_pad.push('-');
        }

        let mut layout_str = String::new();

        println!("{}", self);

        layout_str
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_formatting() {
        let file_a = NsFile::new_file("a", 1234);
        let dir_b = NsFile::new_dir("b");

        assert_eq!("a (file, size=1234)", format!("{}", file_a));
        assert_eq!("b (dir)", format!("{}", dir_b));
    }

    #[test]
    fn create_from_line() {
        let input_data = vec![
            ("dir a", true, 0, "a"),
            ("14848514 b.txt", false, 14848514, "b.txt"),
            ("8504156 c.dat", false, 8504156, "c.dat"),
            ("dir d", true, 0, "d"),
        ];

        for data in input_data {
            let line = data.0;
            let is_dir = data.1;
            let size: usize = data.2;
            let name = data.3;

            let test_file = NsFile::from_line(line);
            assert_eq!(name, test_file.name);
            assert_eq!(is_dir, test_file.is_dir);
            assert_eq!(size, test_file.size);
        }
    }

    #[test]
    fn add_child_to_dir() {
        let mut test_root_dir = NsFile::new_dir("a");
        let test_file = NsFile::new_file("b", 1234);
        let test_dir = NsFile::new_file("c", 5678);

        test_root_dir.add_child(test_file);
        test_root_dir.add_child(test_dir);

        assert_eq!(2, test_root_dir.children.len());
    }
}
