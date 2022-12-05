#[derive(Debug)]
pub struct Rucksack<'a> {
    full_contents: &'a str,
    compartments: (&'a str, &'a str),
    count: usize,
}

impl<'a> Rucksack<'a> {
    /// Creates a new Rucksack from an input string
    pub fn from_string(item_str: &'a str) -> Self {
        let compartments = item_str.split_at(item_str.len() / 2);

        Rucksack {
            full_contents: item_str,
            compartments,
            count: item_str.len(),
        }
    }

    pub fn from_lines(item_lines: &Vec<&'a str>) -> Vec<Self> {
        let mut rucksacks: Vec<Rucksack> = Vec::new();

        for line in item_lines {
            rucksacks.push(Rucksack::from_string(line));
        }

        rucksacks
    }

    pub fn groups_from_lines(item_lines: &Vec<&'a str>) -> Vec<[Self; 3]> {
        let mut groups = Vec::new();

        for i in (0..item_lines.len()).step_by(3) {
            let mut group = [
                Rucksack::from_string(item_lines[i]),
                Rucksack::from_string(item_lines[i + 1]),
                Rucksack::from_string(item_lines[i + 2]),
            ];

            group.sort_by(|a, b| b.count.cmp(&a.count));

            groups.push(group);
        }

        groups
    }

    /// Creates a string from the items that are present in both Rucksack compartments
    pub fn get_overlap(&self) -> Option<char> {
        for i in 0..self.compartments.0.len() {
            let char0 = self.compartments.0.chars().nth(i).unwrap();

            if self.compartments.1.contains(char0) {
                return Some(char0);
            }
        }

        None
    }

    pub fn get_overlap_priority(&self) -> u8 {
        match self.get_overlap() {
            Some(c) => priority_of_char(&c),
            None => 0,
        }
    }

    pub fn get_group_overlap(group: &[Rucksack<'a>; 3]) -> Option<char> {
        for c in group[0].full_contents.chars() {
            if group[1].full_contents.contains(c) && group[2].full_contents.contains(c) {
                return Some(c);
            }
        }

        None
    }

    pub fn get_group_overlap_priority(group: &[Rucksack<'a>; 3]) -> u8 {
        match Rucksack::get_group_overlap(group) {
            Some(c) => priority_of_char(&c),
            None => 0,
        }
    }
}

fn priority_of_char(c: &char) -> u8 {
    let mut priority: u8 = match c.to_ascii_lowercase() {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        val => {
            panic!("Unsupported character: \'{val}\'");
        }
    };

    if c.is_ascii_uppercase() {
        priority += 26;
    }

    priority
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_priority() {
        let input_data: Vec<(char, u8)> = vec![('a', 1), ('A', 27)];

        for data in input_data {
            assert_eq!(data.1, priority_of_char(&data.0));
        }
    }

    mod rucksack {
        use crate::Rucksack;

        #[test]
        fn equal_split() {
            let input_data = vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ];

            for data in input_data {
                let rucksack = Rucksack::from_string(data);

                assert_eq!(rucksack.compartments.0.len(), rucksack.compartments.1.len());
            }
        }

        #[test]
        fn overlap() {
            let input_data = vec![
                ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
                ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
                ("PmmdzqPrVvPwwTWBwg", 'P'),
                ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v'),
                ("ttgJtRGJQctTZtZT", 't'),
                ("CrZsJsPPZsGzwwsLwLmpwMDw", 's'),
            ];

            for data in input_data {
                let rucksack = Rucksack::from_string(data.0);

                assert_eq!(Some(data.1), rucksack.get_overlap());
            }
        }

        #[test]
        fn group_overlap() {
            let input_data = (
                vec![
                    "vJrwpWtwJgWrhcsFMMfFFhFp",
                    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                    "PmmdzqPrVvPwwTWBwg",
                ],
                'r',
            );

            let rucksacks = Rucksack::groups_from_lines(&input_data.0);
            assert_eq!(
                Some(input_data.1),
                Rucksack::get_group_overlap(&rucksacks[0])
            );
        }
    }
}
