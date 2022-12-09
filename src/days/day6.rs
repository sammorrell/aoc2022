#[cfg(test)]
mod tests {
    use crate::io;
    use std::path::Path;

    #[test]
    fn day6_example() {
        let target_window_size: usize = 4;
        let input = io::read_string_col(Path::new("data/day6/example.txt"))
            .expect("Unable to find input file. ");

        let outputs: Vec<usize> = input
            .iter()
            .map(|input_str| {
                input_str
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(target_window_size)
                    .position(|wind| {
                        let mut tmp: Vec<char> = wind.clone().into();
                        tmp.sort();
                        tmp.dedup();
                        tmp.len() == target_window_size
                    })
                    .expect("Valid window not found in input. ")
                    + 4
            })
            .collect();

        assert_eq!(outputs[0], 7);
        assert_eq!(outputs[1], 5);
        assert_eq!(outputs[2], 6);
        assert_eq!(outputs[3], 10);
        assert_eq!(outputs[4], 11);
    }

    #[test]
    fn day6_part1() {
        // Because we are looking for a start of transmission, we use a 4 char window.
        let target_window_size: usize = 4;
        let input = io::read_string_col(Path::new("data/day6/data.txt"))
            .expect("Unable to find input file. ");

        let outputs: Vec<usize> = input
            .iter()
            .map(|input_str| {
                // For each input we take a sliding N-char window, sort and dedup and check to see if it is the
                // same size as the window. If so, we can safely conclude that all characters are unique
                input_str
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(target_window_size)
                    .position(|wind| {
                        let mut tmp: Vec<char> = wind.clone().into();
                        tmp.sort();
                        tmp.dedup();
                        tmp.len() == target_window_size
                    })
                    .expect("Valid window not found in input. ")
                    + target_window_size // Add on here to compensate for widnow size.
            })
            .collect();

        assert_eq!(outputs[0], 1850);
    }

    #[test]
    fn day6_part2() {
        // Now that we are dealing with start of message, we change to a 14 char window.
        let target_window_size: usize = 14;
        let input = io::read_string_col(Path::new("data/day6/data.txt"))
            .expect("Unable to find input file. ");

        let outputs: Vec<usize> = input
            .iter()
            .map(|input_str| {
                // For each input we take a sliding N-char window, sort and dedup and check to see if it is the
                // same size as the window. If so, we can safely conclude that all characters are unique
                input_str
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(target_window_size)
                    .position(|wind| {
                        let mut tmp: Vec<char> = wind.clone().into();
                        tmp.sort();
                        tmp.dedup();
                        tmp.len() == target_window_size
                    })
                    .expect("Valid window not found in input. ")
                    + target_window_size // Add on here to compensate for widnow size.
            })
            .collect();

        assert_eq!(outputs[0], 2823);
    }
}
