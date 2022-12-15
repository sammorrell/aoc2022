use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

const MONKEY_PATTERN: &str = r"Monkey ([0-9]+):\n  Starting items: ([\d, ]+)\n  Operation: ([=\-*/+ \w\d]+)\n  Test: ([ \w\d]+)\n    If true: throw to monkey ([\d]+)\n    If false: throw to monkey ([\d]+)";

#[derive(Debug, Default)]
pub struct Monkey {
    id: usize,
    items: VecDeque<i128>,
    operation: Operation,
    test_divisible: i128,
    false_target: usize,
    true_target: usize,
    inspections: usize,
}

impl Monkey {
    pub fn new() -> Monkey {
        Monkey {
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub enum Operation {
    AddNum(i128),
    MulNum(i128),
    #[default]
    AddOld,
    MulOld,
}

impl Operation {
    pub fn from_string(input: &str) -> Operation {
        let segs: VecDeque<&str> = input.split(" ").collect();
        let operator = segs[3];
        let operand = segs[4];

        if operand == "old" {
            if operator == "+" {
                Operation::AddOld
            } else {
                Operation::MulOld
            }
        } else {
            let val = operand.parse::<i128>().expect("The operator should be a number. ");
            if operator == "+" {
                Operation::AddNum(val)
            } else {
                Operation::MulNum(val)
            }
        }
    }

    pub fn apply(&self, item_val: i128) -> i128 {
        match self {
            Self::AddOld => item_val + item_val,
            Self::MulOld => item_val * item_val,
            Self::AddNum(ref num) => item_val + num,
            Self::MulNum(ref num) => item_val * num,
        }
    }
}

/// A very basic function which parses the required information for this task
/// from the file into a usable data strcuture. 
pub fn monkeys_from_string(string: &String) -> Vec<Monkey> {
    // First, get the items using regex.
    let items_regex = Regex::new(MONKEY_PATTERN).unwrap();
    items_regex
        .captures_iter(string)
        .into_iter()
        .map(|cap| {
            let id = cap[1].parse::<usize>().expect("No ID for monkey. ");
            let items: VecDeque<i128> = cap[2]
                .split(",")
                .map(|item| {
                    item.trim()
                        .parse::<i128>()
                        .expect("Unable to parse int from item. ")
                })
                .collect();
            let operation = Operation::from_string(&cap[3]);
            let test_divisible = cap[4].split(" ").nth(2).unwrap().parse::<i128>().unwrap();
            let true_target = cap[5].parse::<usize>().unwrap();
            let false_target = cap[6].parse::<usize>().unwrap();

            Monkey {
                id,
                items,
                operation,
                test_divisible,
                true_target,
                false_target,
                ..Default::default()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::io;
    use std::path::Path;
    use super::{monkeys_from_string};

    #[test]
    pub fn day11_example() {
        let input_string =
            io::read_string(Path::new("data/day11/example.txt")).expect("Unable to find input. ");
        let mut monkies = monkeys_from_string(&input_string);
        
        for iround in 0..20 {
            for imonkey in 0..monkies.len() {
                // First, do a pass over to test the monkies. 
                let mut inspec = 0;
                monkies[imonkey].items = monkies[imonkey].items.iter().map(|item| { 
                    inspec += 1;
                    monkies[imonkey].operation.apply(*item) / 3
                }).collect();

                monkies[imonkey].inspections += inspec;

                // Now check the items
                while let Some(item) = monkies[imonkey].items.pop_front() {
                    if item % monkies[imonkey].test_divisible == 0 {
                        let itar = monkies[imonkey].true_target;
                        monkies[itar].items.push_back(item);
                    } else {
                        let itar = monkies[imonkey].false_target;
                        monkies[itar].items.push_back(item);
                    }
                }
            }
        }

        let mut inspections: Vec<usize> = monkies.iter().map(|monk| monk.inspections).collect();
        inspections.sort();
        let top2_inspections = inspections.into_iter().rev().take(2).collect::<Vec<usize>>();
        assert_eq!(top2_inspections[0] * top2_inspections[1], 10605);
    }

    #[test]
    pub fn day11_part1() {
        let input_string =
            io::read_string(Path::new("data/day11/data.txt")).expect("Unable to find input. ");
        let mut monkies = monkeys_from_string(&input_string);
        
        for iround in 0..20 {
            for imonkey in 0..monkies.len() {
                // First, do a pass over to test the monkies. 
                let mut inspec = 0;
                monkies[imonkey].items = monkies[imonkey].items.iter().map(|item| { 
                    inspec += 1;
                    monkies[imonkey].operation.apply(*item) / 3
                }).collect();

                monkies[imonkey].inspections += inspec;

                // Now check the items
                while let Some(item) = monkies[imonkey].items.pop_front() {
                    if item % monkies[imonkey].test_divisible == 0 {
                        let itar = monkies[imonkey].true_target;
                        monkies[itar].items.push_back(item);
                    } else {
                        let itar = monkies[imonkey].false_target;
                        monkies[itar].items.push_back(item);
                    }
                }
            }
        }

        let mut inspections: Vec<usize> = monkies.iter().map(|monk| monk.inspections).collect();
        inspections.sort();
        let top2_inspections = inspections.into_iter().rev().take(2).collect::<Vec<usize>>();
        // The answer provided by AOC. 
        assert_eq!(top2_inspections[0] * top2_inspections[1], 51075);
    }
}
