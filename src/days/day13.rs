use std::path::Path;
use serde_derive::{Deserialize, Serialize};
use serde_json as json;
use crate::io::read_text_chunks;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PacketItem {
    Integer(usize),
    List(Vec<PacketItem>),
}

pub fn parse_input(input_file: &Path) -> Vec<(PacketItem, PacketItem)> {
    let packet_chunks = read_text_chunks(input_file).expect("Unable to read input file. ");
    packet_chunks.iter().map(|chunk| {
        let left: PacketItem = json::from_str(&chunk[0]).expect("Unable to parse left. ");
        let right: PacketItem = json::from_str(&chunk[1]).expect("Unable to parse right. ");
        (left, right)
    })
    .collect()
}

pub fn in_correct_order(left: &PacketItem, right: &PacketItem) -> Option<bool> {
    match (left, right) {
        // The case that both are integers. 
        (PacketItem::Integer(li), PacketItem::Integer(ri)) => {
            if li < ri {
                Some(true)
            } else {
                if li == ri {
                    None
                } else {
                    Some(false)
                }
            }
        },
        // The case that both are lists, we iterate and handle them off to handle each item. 
        (PacketItem::List(ll), PacketItem::List(rl)) => {
            // Iterate through the maximum of both lists, so that we can check if one is shorter. 
            (0..ll.len().max(rl.len())).map(|idx| {
                match ll.iter().nth(idx) {
                    Some(left_child) => {
                        match rl.iter().nth(idx) {
                            Some(right_child) => {
                                in_correct_order(left_child, right_child)
                            },
                            // Of the right hand lister is shorter, this is not sorted.
                            None => Some(false),
                        }
                    },
                    // If the left hand list is shorter, it is sorted. 
                    None => Some(true),
                }
            })
            .into_iter()
            .find(Option::is_some)
            .unwrap_or(None)
        },
        // Now handle the left being a list, and right being an int case. 
        (PacketItem::List(_), PacketItem::Integer(ri)) => in_correct_order(left, &PacketItem::List(vec![PacketItem::Integer(*ri)])),
        (PacketItem::Integer(li), PacketItem::List(_)) => in_correct_order(&PacketItem::List(vec![PacketItem::Integer(*li)]), right),
        _ => panic!("Unexpected pattern in packet comarison. ")
    }  
}

#[cfg(test)]
mod tests {
    use super::{parse_input, in_correct_order};
    use std::path::Path;

    #[test]
    fn day13_example() {
        let packet_pairs = parse_input(Path::new("data/day13/example.txt"));
        let indices: Vec<usize> = packet_pairs
            .iter()
            .enumerate()
            .map(|(idx, (left, right))| {
                match in_correct_order(left, right) {
                    Some(test) => if test == true { Some(idx + 1) } else { None }, 
                    None => None,
                }
            })
            .filter(Option::is_some)
            .map(|index_opt| index_opt.unwrap())
            .collect();
        
        let index_sum: usize = indices.iter().sum();
        assert_eq!(index_sum, 13);
    }

    #[test]
    fn day13_part1() {
        let packet_pairs = parse_input(Path::new("data/day13/data.txt"));
        let indices: Vec<usize> = packet_pairs
            .iter()
            .enumerate()
            .map(|(idx, (left, right))| {
                match in_correct_order(left, right) {
                    Some(test) => if test == true { Some(idx + 1) } else { None }, 
                    None => None,
                }
            })
            .filter(Option::is_some)
            .map(|index_opt| index_opt.unwrap())
            .collect();
        
        let index_sum: usize = indices.iter().sum();
        assert_eq!(index_sum, 5623);
    }
}