use std::fmt::{Display, Formatter};

/// Basic 'File' implementation for the puzzle
pub struct NsFile<'a> {
    /// Name of the file or directory
    name: &'a str,
    /// Size (in bytes) of the NsFile. Will be 0 if the instance is a directory
    size: usize,
    /// Flag specifying if this instance is a directory and can have children
    is_dir: bool,
    /// Child NsFile instances
    children: Vec<&'a NsFile<'a>>,
}

impl Display for NsFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        return if self.is_dir {
            write!(f, "{} (dir)", self.name)
        } else {
            write!(f, "{} (file, size={})", self.name, self.size)
        };
    }
}

impl<'a> NsFile<'a> {
    /// Create a new named file
    pub fn new_file(name: &'a str, size: usize) -> Self {
        NsFile {
            name,
            size,
            is_dir: false,
            children: Vec::new(),
        }
    }

    /// Create a new named directory
    pub fn new_dir(name: &'a str) -> Self {
        NsFile {
            name,
            size: 0,
            is_dir: true,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: &'a NsFile) {
        if !self.is_dir {
            // Children can only be added to directories
            return;
        }

        self.children.push(child);
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
}
