#[derive(Debug)]
pub struct SectionAssignment {
    start: usize,
    end: usize,
}

impl SectionAssignment {
    pub fn from_string(input: &str) -> Self {
        let splits = input.split('-').collect::<Vec<&str>>();
        let start = splits[0].parse().unwrap();
        let end = splits[1].parse().unwrap();

        SectionAssignment { start, end }
    }

    pub fn pair_from_line(input: &str) -> [Self; 2] {
        let splits = input.split(',').collect::<Vec<&str>>();

        [
            SectionAssignment::from_string(splits[0]),
            SectionAssignment::from_string(splits[1]),
        ]
    }

    pub fn has_full_overlap(pair: &[SectionAssignment; 2]) -> bool {
        if pair[0].start >= pair[1].start && pair[0].end <= pair[1].end {
            true
        } else if pair[1].start >= pair[0].start && pair[1].end <= pair[0].end {
            true
        } else {
            false
        }
    }

    pub fn has_partial_overlap(pair: &[SectionAssignment; 2]) -> bool {
        if pair[0].start >= pair[1].start && pair[0].start <= pair[1].end {
            true
        } else if pair[1].start >= pair[0].start && pair[1].start <= pair[0].end {
            true
        } else if pair[0].end <= pair[1].end && pair[0].end >= pair[1].start {
            true
        } else if pair[1].end <= pair[0].end && pair[1].end >= pair[0].start {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    mod section_assignment {
        use crate::SectionAssignment;

        #[test]
        fn create_from_string() {
            let input_data = [("2-4", 2, 4), ("6-8", 6, 8)];

            for data in input_data {
                let section_assignment = SectionAssignment::from_string(data.0);
                assert_eq!(data.1, section_assignment.start);
                assert_eq!(data.2, section_assignment.end);
            }
        }

        #[test]
        fn create_pair_from_string() {
            let input_data = [("2-4,6-8", [(2, 4), (6, 8)])];

            for data in input_data {
                let assignment_pair = SectionAssignment::pair_from_line(data.0);
                for i in 0..2 {
                    assert_eq!(data.1[i].0, assignment_pair[i].start);
                    assert_eq!(data.1[i].1, assignment_pair[i].end);
                }
            }
        }

        #[test]
        fn full_overlap() {
            let input_data = [("2-4,6-8", false), ("4-8,1-9", true), ("1-9,4-8", true)];

            for data in input_data {
                let assignment_pair = SectionAssignment::pair_from_line(data.0);
                assert_eq!(
                    data.1,
                    SectionAssignment::has_full_overlap(&assignment_pair)
                );
            }
        }

        #[test]
        fn partial_overlap() {
            let input_data = [
                ("2-4,6-8", false),
                ("4-8,1-9", true),
                ("1-9,4-8", true),
                ("5-7,7-9", true),
                ("2-6,4-8", true),
                ("4-8,2-6", true),
            ];

            for data in input_data {
                let assignment_pair = SectionAssignment::pair_from_line(data.0);
                assert_eq!(
                    data.1,
                    SectionAssignment::has_partial_overlap(&assignment_pair)
                );
            }
        }
    }
}
