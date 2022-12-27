use crate::io::read_text_chunks;
use serde_derive::{Deserialize, Serialize};
use serde_json as json;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PacketItem {
    Integer(usize),
    List(Vec<PacketItem>),
}

pub fn parse_input(input_file: &Path) -> Vec<(PacketItem, PacketItem)> {
    let packet_chunks = read_text_chunks(input_file).expect("Unable to read input file. ");
    packet_chunks
        .iter()
        .map(|chunk| {
            let left: PacketItem = json::from_str(&chunk[0]).expect("Unable to parse left. ");
            let right: PacketItem = json::from_str(&chunk[1]).expect("Unable to parse right. ");
            (left, right)
        })
        .collect()
}

/// Checks to see whether the current PacketItem is our divider item.
pub fn is_decoder_packet(packet: &PacketItem) -> bool {
    const DIVIER_INTS: [usize; 2] = [2_usize, 6_usize];
    if let PacketItem::List(layer1) = packet {
        if let Some(PacketItem::List(layer2)) = layer1.first() {
            if let Some(PacketItem::Integer(intval)) = layer2.first() {
                if layer1.len() == 1 && layer2.len() == 1 && DIVIER_INTS.contains(intval) {
                    return true;
                }
            }
        }
    }
    false
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
        }
        // The case that both are lists, we iterate and handle them off to handle each item.
        (PacketItem::List(ll), PacketItem::List(rl)) => {
            // Iterate through the maximum of both lists, so that we can check if one is shorter.
            (0..ll.len().max(rl.len()))
                .map(|idx| {
                    match ll.iter().nth(idx) {
                        Some(left_child) => {
                            match rl.iter().nth(idx) {
                                Some(right_child) => in_correct_order(left_child, right_child),
                                // Of the right hand lister is shorter, this is not sorted.
                                None => Some(false),
                            }
                        }
                        // If the left hand list is shorter, it is sorted.
                        None => Some(true),
                    }
                })
                .into_iter()
                .find(Option::is_some)
                .unwrap_or(None)
        }
        // Now handle the left being a list, and right being an int case.
        (PacketItem::List(_), PacketItem::Integer(ri)) => {
            in_correct_order(left, &PacketItem::List(vec![PacketItem::Integer(*ri)]))
        }
        (PacketItem::Integer(li), PacketItem::List(_)) => {
            in_correct_order(&PacketItem::List(vec![PacketItem::Integer(*li)]), right)
        }
        _ => panic!("Unexpected pattern in packet comarison. "),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::map;

    use crate::days::day13::{is_decoder_packet, PacketItem};

    use super::{in_correct_order, parse_input};
    use std::path::Path;

    #[test]
    fn day13_example() {
        let packet_pairs = parse_input(Path::new("data/day13/example.txt"));
        let indices: Vec<usize> = packet_pairs
            .iter()
            .enumerate()
            .map(|(idx, (left, right))| match in_correct_order(left, right) {
                Some(test) => {
                    if test == true {
                        Some(idx + 1)
                    } else {
                        None
                    }
                }
                None => None,
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
            .map(|(idx, (left, right))| match in_correct_order(left, right) {
                Some(test) => {
                    if test == true {
                        Some(idx + 1)
                    } else {
                        None
                    }
                }
                None => None,
            })
            .filter(Option::is_some)
            .map(|index_opt| index_opt.unwrap())
            .collect();

        let index_sum: usize = indices.iter().sum();
        assert_eq!(index_sum, 5623);
    }

    #[test]
    fn day13_part2_example() {
        let packet_pairs = parse_input(Path::new("data/day13/example.txt"));
        let divider_packets: Vec<PacketItem> =
            parse_input(Path::new("data/day13/divider_packets.txt"))
                .into_iter()
                .map(|(r, l)| vec![r, l])
                .flatten()
                .collect();

        let mut packets: Vec<PacketItem> = packet_pairs
            .into_iter()
            .map(|(r, l)| vec![r, l])
            .flatten()
            .chain(divider_packets)
            .collect();

        // Let's do a bubble sort with the testing code we just wrote.
        let n_packet = packets.len();
        for i in 0..(n_packet - 1) {
            for j in 0..(n_packet - i - 1) {
                match in_correct_order(&packets[j], &packets[j + 1]) {
                    Some(test) => {
                        if !test {
                            packets.swap(j, j + 1);
                        }
                    }
                    None => panic!("What do I do!?"),
                }
            }
        }

        // Now find the divider packets
        let decoder_key: usize = packets
            .iter()
            .enumerate()
            .map(|(idx, packet)| {
                if is_decoder_packet(packet) {
                    Some(idx + 1)
                } else {
                    None
                }
            })
            .filter(Option::is_some)
            .map(|val| val.unwrap())
            .product();

        assert_eq!(decoder_key, 140);
    }

    #[test]
    fn day13_part2() {
        let packet_pairs = parse_input(Path::new("data/day13/data.txt"));
        let divider_packets: Vec<PacketItem> =
            parse_input(Path::new("data/day13/divider_packets.txt"))
                .into_iter()
                .map(|(r, l)| vec![r, l])
                .flatten()
                .collect();

        let mut packets: Vec<PacketItem> = packet_pairs
            .into_iter()
            .map(|(r, l)| vec![r, l])
            .flatten()
            .chain(divider_packets)
            .collect();

        // Let's do a bubble sort with the testing code we just wrote.
        let n_packet = packets.len();
        for i in 0..(n_packet - 1) {
            for j in 0..(n_packet - i - 1) {
                match in_correct_order(&packets[j], &packets[j + 1]) {
                    Some(test) => {
                        if !test {
                            packets.swap(j, j + 1);
                        }
                    }
                    None => panic!("What do I do!?"),
                }
            }
        }

        // Now find the divider packets
        let decoder_key: usize = packets
            .iter()
            .enumerate()
            .map(|(idx, packet)| {
                if is_decoder_packet(packet) {
                    Some(idx + 1)
                } else {
                    None
                }
            })
            .filter(Option::is_some)
            .map(|val| val.unwrap())
            .product();

        assert_eq!(decoder_key, 20570);
    }
}
