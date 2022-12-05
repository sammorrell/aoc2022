use std::collections::HashSet;

use crate::{
    io::read_string_col
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Rucksack {
    pub compartment_items: Vec<Vec<char>>,
}

impl Rucksack {
    pub fn new_two_comparments_from_string(input: &String) -> Self {
        let comp_len = input.len() / 2;
        let compartment_items = input
            .chars()
            .collect::<Vec<char>>()
            .chunks(comp_len)
            .map(|chunk| Vec::from(chunk))
            .collect::<Vec<_>>();

        Rucksack { compartment_items }
    }

    pub fn items_in_all_compartments(&self) -> Vec<char> {
        self.compartment_items.iter().map(|comp| comp.clone() ).fold(Vec::new(), |accum, comp| {
            if accum.is_empty() {
                comp.iter().map(|it| it.clone() ).collect::<Vec<char>>()
            } else {
                let comp_set = comp.clone().into_iter().collect::<HashSet<_>>();
                comp_set.intersection(&accum.clone().into_iter().collect::<HashSet<_>>()).map(|it| it.clone()).collect::<Vec<char>>()
            }
        }).to_vec()
    }
}

pub fn total_priority(chars: Vec<char>) -> usize {
    chars.iter().map(|item| item_priority(item) ).sum::<usize>()
}

pub fn item_priority(item: &char) -> usize {
    if item.is_ascii_lowercase() {
        *item as usize - 'a' as usize + 1
    } else {
        *item as usize - 'A' as usize + 27
    }
}

#[cfg(test)]
mod tests {
    use super::{Rucksack, total_priority};
    use crate::{read_string_col};
    use std::path::Path;

    #[test]
    fn day3_part1() {
        let input = read_string_col(Path::new("data/day3/data.txt")).unwrap();
        let rucksacks: Vec<Rucksack> = input.iter().map(|content| Rucksack::new_two_comparments_from_string(content)).collect();
        let tot = rucksacks.iter().map(|rs| {
            total_priority(rs.items_in_all_compartments())
        }).sum::<usize>();

        // Checking against the answer from AOC. 
        assert_eq!(tot, 8240);
    }
}