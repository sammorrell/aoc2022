use itertools::Itertools;
use regex::Regex;

use crate::err::Error;

const REARRANGEMENT_PATTERN: &str = "move (\\d+) from (\\d+) to (\\d+)";

/// The rearrangement struct, which is responsible for paring and carring out
/// rearrangement operations on our stacks structure.
#[derive(Debug, Clone)]
pub struct Rearrangement {
    pub from_stack: usize,
    pub to_stack: usize,
    pub n: usize,
}

impl Rearrangement {
    pub fn from_string(in_string: &String) -> Result<Rearrangement, Error> {
        let pattern = Regex::new(REARRANGEMENT_PATTERN).unwrap();
        let captures = pattern.captures(in_string).unwrap();

        let n = captures[1].parse::<usize>()?;
        let from_stack = captures[2].parse::<usize>()?;
        let to_stack = captures[3].parse::<usize>()?;
        Ok(Rearrangement {
            from_stack,
            to_stack,
            n,
        })
    }

    /// This is the method implemented for part 1, which pops and pushes crates one at a time.
    pub fn rearrange(&self, stacks: &mut Vec<Vec<char>>) {
        for _ in 0..self.n {
            let tmp = stacks[self.from_stack - 1].pop().unwrap();
            stacks[self.to_stack - 1].push(tmp);
        }
    }

    /// This is the function implemented for part 2 where we can move N crates at once in the
    /// same order.
    pub fn rearrange_multiple(&self, stacks: &mut Vec<Vec<char>>) {
        let tmp = (0..self.n)
            .map(|_| stacks[self.from_stack - 1].pop().unwrap())
            .collect::<Vec<char>>();
        for ch in tmp.iter().rev() {
            stacks[self.to_stack - 1].push(ch.clone());
        }
    }
}

/// Parses the rearrangements segments form our input file.
pub fn parse_rearrangements(lines: &Vec<String>) -> Vec<Rearrangement> {
    lines
        .iter()
        .map(|line| Rearrangement::from_string(line).unwrap())
        .collect::<Vec<Rearrangement>>()
}

/// Parses the stacks segment from our input file.
pub fn parse_stacks(lines: &Vec<String>) -> Vec<Vec<char>> {
    let box_re = Regex::new("\\[([A-Z]{1})\\]").unwrap();

    // First, parse the lines into char arrays using regex.
    // We must remember that blank spaces need to be represented too, for which we use None.
    let parsed_lines: Vec<Vec<Option<char>>> = lines
        .iter()
        .rev()
        .skip(1)
        .map(|line| {
            line.chars().chunks(4).into_iter().fold(
                Vec::<Option<char>>::new(),
                |mut stack, chars| {
                    let string = chars.collect::<String>();
                    if box_re.is_match(&string) {
                        stack.push(Some(string.chars().nth(1).unwrap()))
                    } else {
                        stack.push(None)
                    }
                    stack
                },
            )
        })
        .collect();

    // Initialise our stacks data structure, ensuring to give enough capacity for all stacks.
    // We make the reasonable assumption, that each line has the same capacity.
    let mut stacks: Vec<Vec<char>> = (0..parsed_lines.first().unwrap().len())
        .into_iter()
        .map(|_| Vec::new())
        .collect_vec();

    // Now iterate through the lines and fille the data strcuture, skipping stacks where there was a None.
    for i_row in 0..parsed_lines.len() {
        let curr_line = &parsed_lines[i_row];
        for (i_stack, position) in curr_line.iter().enumerate() {
            if position.is_some() {
                stacks[i_stack].push(position.unwrap().clone());
            }
        }
    }
    stacks
}

/// Responsible for parsing all of the input.
/// Practically, this does a split operation on the blank line and hands off each
/// segment to a dedicated function.
pub fn parse_input(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<Rearrangement>) {
    let isplit = lines.iter().position(|l| *l == "").unwrap();
    let stack_lines: Vec<String> = lines[0..isplit].into();
    let rearrange_lines: Vec<String> = lines[isplit + 1..].into();

    (
        parse_stacks(&stack_lines),
        parse_rearrangements(&rearrange_lines),
    )
}

#[cfg(test)]
mod tests {
    use crate::io;
    use std::path::Path;

    use super::parse_input;

    #[test]
    fn day5_example() {
        let lines = io::read_string_col(Path::new("data/day5/example.txt")).unwrap();
        let (mut stacks, rearrangements) = parse_input(&lines);
        for r in rearrangements {
            r.rearrange(&mut stacks);
        }

        // Now we test against the example input.
        let outstr = stacks
            .into_iter()
            .map(|curr_stack| *curr_stack.last().as_ref().unwrap().clone())
            .collect::<String>();
        assert_eq!(outstr, "CMZ".to_string());
    }

    #[test]
    fn day5_part1() {
        let lines = io::read_string_col(Path::new("data/day5/data.txt")).unwrap();
        let (mut stacks, rearrangements) = parse_input(&lines);
        for r in rearrangements {
            r.rearrange(&mut stacks);
        }

        // Now we get the output and test against the answer provided by AOC.
        let outstr = stacks
            .into_iter()
            .map(|curr_stack| *curr_stack.last().as_ref().unwrap().clone())
            .collect::<String>();
        assert_eq!(outstr, "SBPQRSCDF".to_string());
    }

    #[test]
    fn day5_part2() {
        let lines = io::read_string_col(Path::new("data/day5/data.txt")).unwrap();
        let (mut stacks, rearrangements) = parse_input(&lines);
        for r in rearrangements {
            r.rearrange_multiple(&mut stacks);
        }

        // Now we get the output and test against the answer provided by AOC.
        let outstr = stacks
            .into_iter()
            .map(|curr_stack| *curr_stack.last().as_ref().unwrap().clone())
            .collect::<String>();
        assert_eq!(outstr, "RGLVRCQSB".to_string());
    }
}
