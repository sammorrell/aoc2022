use itertools::Itertools;
use std::cmp::{min, max};

use crate::io::read_string_col;
use std::path::Path;
const CAVE_DIMS: (usize, usize) = (1000, 1000);

#[derive(Debug, PartialEq, Eq)]
pub enum CavePoint {
    Air, 
    Rock, 
    Sand,
}

pub fn parse_coords_set(instr: &String) -> Vec<(usize, usize)> {
    instr.split("->")
        .into_iter()
        .map(|segment_str| {
            let segs: Vec<&str> = segment_str.split(",").collect();
            let x = segs[0].trim().parse::<usize>().expect("Unable to parse x-coordinate. ");
            let y = segs[1].trim().parse::<usize>().expect("Unable to parse x-coordinate. ");
            (y, x)
        })
        .collect()
}

pub fn make_cave(rocks_path: &Path) -> Vec<Vec<CavePoint>> {
    let mut cave: Vec<Vec<CavePoint>> = (0..CAVE_DIMS.0)
        .map(|i| {
            (0..CAVE_DIMS.1).map(|j| CavePoint::Air).collect()
        })
        .collect();

    for rock_str in read_string_col(rocks_path).expect("Unable to load rocks from file. ") {
        for (prev, curr) in parse_coords_set(&rock_str).iter().tuple_windows() {
            if prev.0 == curr.0 {
                for j in min(prev.1, curr.1)..max(prev.1, curr.1) {
                    cave[curr.0][j] = CavePoint::Rock;
                }
            } else {
                for i in min(prev.0, curr.0)..max(prev.0, curr.0) {
                    cave[i][curr.1] = CavePoint::Rock;
                }
            }
            cave[prev.0][prev.1] = CavePoint::Rock;
            cave[curr.0][curr.1] = CavePoint::Rock;
        };
    }
    
    cave
}

pub fn add_cave_floor(rocks_path: &Path, cave: &mut Vec<Vec<CavePoint>>) {
    let rock_str = read_string_col(rocks_path).expect("Unable to load rocks from file. ");
    let floor_level = rock_str
        .iter()
        .map(|instr| {
            parse_coords_set(instr)
            .iter()
            .map(|(i, _j)| i.clone() )
            .collect::<Vec<usize>>()
        })
        .flatten()
        .max()
        .unwrap() + 2;

    println!("Adding floor at level {}", floor_level);

    for j in 0..CAVE_DIMS.1 {
        cave[floor_level][j] = CavePoint::Rock;
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day14::add_cave_floor;

    use super::{make_cave, CavePoint};
    use std::path::Path;

    #[test]
    fn day14_part1() {
        let mut cave = make_cave(Path::new("data/day14/data.txt"));
        let start_point = (0_usize, 500_usize);
        let abyss_level = 999_usize;
        let mut abyss_reached = false;
        let mut n_sand_rest = 0_usize;
        
        while !abyss_reached{
            let mut sand_coord = start_point.clone();

            loop {

                if sand_coord.0 >= abyss_level {
                    abyss_reached = true;
                    break;
                }

                let down = (sand_coord.0 + 1, sand_coord.1);
                if cave[down.0][down.1] == CavePoint::Air {
                    sand_coord = down;
                    continue;
                }

                let left = (sand_coord.0 + 1, sand_coord.1 - 1);
                if cave[left.0][left.1] == CavePoint::Air {
                    sand_coord = left;
                    continue;
                }

                let right = (sand_coord.0 + 1, sand_coord.1 + 1);
                if cave[right.0][right.1] == CavePoint::Air {
                    sand_coord = right;
                    continue;
                }

                // The sand has come to rest.
                cave[sand_coord.0][sand_coord.1] = CavePoint::Sand;
                n_sand_rest += 1;
                break;

            }
            println!("{}", n_sand_rest);
        }

        // 1068 is the correct answer accordin to AOC. 
        assert_eq!(n_sand_rest, 1068);
    }

    #[test]
    fn day14_part2() {
        let mut cave = make_cave(Path::new("data/day14/data.txt"));
        add_cave_floor(Path::new("data/day14/data.txt"), &mut cave);

        let start_point = (0_usize, 500_usize);
        let end_coord = start_point;
        let mut start_reached = false;
        let mut n_sand_rest = 0_usize;
        
        while !start_reached{
            let mut sand_coord = start_point.clone();

            loop {

                let down = (sand_coord.0 + 1, sand_coord.1);
                if cave[down.0][down.1] == CavePoint::Air {
                    sand_coord = down;
                    continue;
                }

                let left = (sand_coord.0 + 1, sand_coord.1 - 1);
                if cave[left.0][left.1] == CavePoint::Air {
                    sand_coord = left;
                    continue;
                }

                let right = (sand_coord.0 + 1, sand_coord.1 + 1);
                if cave[right.0][right.1] == CavePoint::Air {
                    sand_coord = right;
                    continue;
                }

                // The sand has come to rest.
                cave[sand_coord.0][sand_coord.1] = CavePoint::Sand;
                n_sand_rest += 1;

                if sand_coord == end_coord {
                    start_reached = true;
                }
                break;
            }
        }

        // 1068 is the correct answer accordin to AOC. 
        assert_eq!(n_sand_rest, 27936);
    }
}