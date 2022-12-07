use std::{collections::HashSet, ops::Range};

pub fn get_marker_range(message: &str) -> Option<Range<usize>> {
    let packet_size = 4;

    for i in 0..message.len() {
        let end = i + packet_size;
        let marker_slice = &message[i..end];
        debug_assert!(marker_slice.len() == packet_size);

        // Check if all characters in the slice are unique;
        let mut char_set = HashSet::new();
        for c in marker_slice.chars() {
            char_set.insert(c);
        }
        if char_set.len() == packet_size {
            return Some(i..end);
        }
    }

    None
}

pub fn get_start_of_message_range(message: &str) -> Option<Range<usize>> {
    let packet_size = 14;

    for i in 0..message.len() {
        let end = i + packet_size;
        let marker_slice = &message[i..end];
        debug_assert!(marker_slice.len() == packet_size);

        // Check if all characters in the slice are unique;
        let mut char_set = HashSet::new();
        for c in marker_slice.chars() {
            char_set.insert(c);
        }
        if char_set.len() == packet_size {
            return Some(i..end);
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_marker_range() {
        let input_data = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(3..7)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(1..5)),
        ];

        for data in input_data {
            let test_str = data.0;
            let test_range = data.1;

            assert_eq!(get_marker_range(test_str), test_range);
        }
    }

    #[test]
    fn test_get_start_of_message_range() {
        let input_data = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(5..19)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(9..23)),
        ];

        for data in input_data {
            let test_str = data.0;
            let test_range = data.1;

            assert_eq!(get_start_of_message_range(test_str), test_range);
        }
    }
}
