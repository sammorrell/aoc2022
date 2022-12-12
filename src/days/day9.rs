use nalgebra::Vector2;

type Real = f64;
type Vec2 = Vector2<Real>;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn unit_vector(&self) -> Vec2 {
        match self {
            Self::Up => Vec2::new(0.0, 1.0),
            Self::Right => Vec2::new(1.0, 0.0),
            Self::Down => Vec2::new(0.0, -1.0),
            Self::Left => Vec2::new(-1.0, 0.0),
        }
    }
}

impl From<String> for Direction {
    fn from(instr: String) -> Self {
        match instr.as_str() {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("Invalid direction in parse. "),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Motion {
    dir: Direction,
    steps: usize,
}

impl Motion {
    pub fn from_str(input: &str) -> Motion {
        let segs: Vec<&str> = input.split(" ").collect();
        let dir = Direction::from(segs[0].to_string());
        let steps = segs[1].parse::<usize>().expect("Unable to parse input. ");
        Motion { dir, steps }
    }

    pub fn move_head(&self, rope: &mut Rope, grid: &mut Grid) {
        let unit_vec = self.dir.unit_vector();
        for _ in 0..self.steps {
            rope.head += unit_vec;
            rope.tail_follow_head();

            // Now check to see if head position is already in visited heads.
            if !grid.visited_head.contains(&rope.head) {
                grid.visited_head.push(rope.head.clone());
            }

            // Now check the visited tails to see if the location already exists.
            if !grid.visited_tail.contains(&rope.tail) {
                grid.visited_tail.push(rope.tail.clone());
            }
        }
    }
}

#[derive(Default)]
pub struct Rope {
    pub head: Vec2,
    pub tail: Vec2,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            ..Default::default()
        }
    }

    pub fn diff_to_motion(diff: Vec2) -> Vec2 {
        diff.map(|comp| {
            if comp == 0.0 {
                0.0
            } else if comp > 0.0 {
                1.0
            } else {
                -1.0
            }
        })
    }

    pub fn tail_follow_head(&mut self) {
        let diff = self.head - self.tail;
        if diff.norm() > 2.0_f64.sqrt() {
            let translate = Self::diff_to_motion(diff);
            self.tail += translate;
        }
    }
}

pub fn parse_motions(in_vec: &Vec<String>) -> Vec<Motion> {
    in_vec
        .into_iter()
        .map(|string| Motion::from_str(string))
        .collect()
}

pub struct Grid {
    width: usize,
    height: usize,
    pub visited_head: Vec<Vec2>,
    pub visited_tail: Vec<Vec2>,
}

impl Grid {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            width,
            height,
            visited_head: vec![],
            visited_tail: vec![],
        }
    }

    pub fn unique_tail_visits(&self) -> usize {
        let tmp = self.visited_tail.clone();
        tmp.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::io;
    use std::path::Path;

    use super::{parse_motions, Grid, Rope};

    #[test]
    pub fn day9_example() {
        let input =
            io::read_string_col(Path::new("data/day9/example.txt")).expect("No lines in input. ");
        let motions = parse_motions(&input);
        let mut rope = Rope::new();
        //rope.head = Vec2::new(4.0, 0.0);
        //rope.tail = Vec2::new(4.0, 0.0);
        let mut grid = Grid::new(5, 6);

        for m in motions {
            m.move_head(&mut rope, &mut grid);
        }

        assert_eq!(grid.unique_tail_visits(), 13);
    }

    #[test]
    pub fn day9_part1() {
        let input =
            io::read_string_col(Path::new("data/day9/data.txt")).expect("No lines in input. ");
        let motions = parse_motions(&input);
        let mut rope = Rope::new();
        //rope.head = Vec2::new(4.0, 0.0);
        //rope.tail = Vec2::new(4.0, 0.0);
        let mut grid = Grid::new(5, 6);

        for m in motions {
            m.move_head(&mut rope, &mut grid);
        }

        assert_eq!(grid.unique_tail_visits(), 6284);
    }
}
