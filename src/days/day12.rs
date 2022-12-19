// This one I got stuck on, as I have not done decision maths for quite a while.
// However I looked through the other solutions and was inspired by:
// https://github.com/frjonsen/aoc2022/blob/master/day12/part2/src/main.rs#L211

use std::{ops::{Add, Sub}, rc::Rc};
use property::Property;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    pub fn neighbours(&self, grid: &Grid) -> Vec<Coord> {
        let mut neighbours = vec![];
        if self.0 > 0 {
            neighbours.push(self - Coord(1, 0))
        }
        if self.1 > 0 {
            neighbours.push(self - Coord(0, 1))
        }
        if self.0 + 1 < grid.ni() {
            neighbours.push(self + Coord(1, 0))
        }
        if self.1 + 1 < grid.nj() {
            neighbours.push(self + Coord(0, 1))
        }
        
        neighbours
    }
}

impl Add<Coord> for &Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Coord> for &Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridNode {
    Height(u8),
    Start,
    End,
}

impl From<GridNode> for i32 {
    fn from(n: GridNode) -> Self {
        match n {
            GridNode::Height(height) => height as i32,
            GridNode::Start => 0,
            GridNode::End => 25, 
        }
    }
}

impl Sub<GridNode> for GridNode {
    type Output = i32;
    fn sub(self, rhs: GridNode) -> Self::Output {
        <i32 as From<GridNode>>::from(self) - <i32 as From<GridNode>>::from(rhs)
    }
}

#[derive(Property, Debug)]
#[property(get(public), set(public), mut(public))]
pub struct Grid {
    data: Vec<Vec<GridNode>>,
    ni: usize, 
    nj: usize,
}

impl Grid {
    pub fn from_string(input: &str) -> Grid {
        // First, get the size of the grid. 
        let lines: Vec<&str> = input.lines().collect();
        let ni = lines.iter().count();
        let nj = lines.first().expect("No input in text when counting row lenth. ").len();

        let grid = (0..ni).map(|i| {
            (0..nj).map(|j| {
                match lines[i].chars().nth(j).unwrap() {
                    'S' => GridNode::Start,
                    'E' => GridNode::End,
                    val => GridNode::Height(val as u8 - 'a' as u8),
                }
            }).collect()
        }).collect();

        Grid {
            data: grid,
            ni, 
            nj
        }
    }

    pub fn end_coord(&self) -> Coord {
        for (i, row) in self.data.iter().enumerate() {
            for (j, node) in row.iter().enumerate() {
                if *node == GridNode::End {
                    return Coord(i, j)
                }
            }
        }

        panic!("Unable to find end node. ")
    }

    pub fn start_coord(&self) -> Coord {
        for (i, row) in self.data.iter().enumerate() {
            for (j, node) in row.iter().enumerate() {
                if *node == GridNode::Start {
                    return Coord(i, j)
                }
            }
        }

        panic!("Unable to find end node. ")
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    parent: Option<Rc<Node>>,
    coord: Coord,
    g: usize, 
    h: usize,
}

impl Node {
    pub fn new(parent: Option<Rc<Node>>, coord: &Coord) -> Node {
        Node { parent, coord: coord.clone(), g: 0, h: 0 }
    }

    pub fn f(&self) -> usize {
        self.g + self.h
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord
    }
}

pub fn can_step(curr_coord: &Coord, next_coord: &Coord, grid: &Grid) -> bool {
    let height_diff = grid.data()[next_coord.1][next_coord.0] - grid.data()[curr_coord.1][curr_coord.0];
    height_diff <= 1
}

pub fn find_path_with_a_star(grid: &Grid, start: &Coord, end: &Coord) -> Option<Vec<Coord>> {
    let mut open_list: Vec<Rc<Node>> = vec![];
    let mut came_from: Vec<Rc<Node>> = vec![];

    // Push the starting point onto the closed list to begin. 
    let start_node = Node::new(None, start);
    open_list.push(Rc::new(start_node));

    while !open_list.is_empty() {
        let (current_node_idx, _) = open_list
            .iter()
            .enumerate()
            .min_by_key(|(_, node)| node.f() )
            .unwrap();

        // Remove current node from open list. 
        let curr_node = {
            let curr_node = open_list.swap_remove(current_node_idx);
            came_from.push(curr_node);
            came_from.last().unwrap()
        };

        // Test to see if we are done. 
        if curr_node.coord == *end {
            let mut final_path = vec![];
            let mut curr = curr_node.clone();
            while curr.parent.is_some() {
                final_path.push(curr.coord.clone());
                curr = curr.parent.clone().unwrap();
            }
            return Some(final_path)
        }

        let neighbours = curr_node.coord.neighbours(grid)
            .into_iter()
            .filter(|neigh| can_step(&curr_node.coord, neigh, grid))
            .map(|neigh| {
                Rc::new(
                    Node {
                        parent: Some(curr_node.clone()),
                        h: end.0.abs_diff(neigh.0) + end.1.abs_diff(neigh.1),
                        coord: neigh,
                        g: curr_node.g + 1,
                    }
                )
            });

        for neighbour in neighbours {
            println!("{:?}", neighbour);
            if came_from.contains(&neighbour) { continue; }
            
            for point in open_list.iter() {
                if point.coord == neighbour.coord && neighbour.g > point.g {
                    continue;
                }
            }
            open_list.push(neighbour);
        }
    };

    None
}

#[cfg(test)]
mod tests {
    use crate::days::day12::find_path_with_a_star;

    use super::Grid;


    #[test]
    pub fn day12_example() {
        let grid = Grid::from_string(include_str!("../../data/day12/example.txt"));
        let start_coord = grid.start_coord();
        let end_coord = grid.end_coord();
        println!("{:?} -> {:?}", start_coord, end_coord);
        println!("{:?}", grid.data());

        let route = find_path_with_a_star(&grid, &start_coord, &end_coord).unwrap();
        assert_eq!(route.len(), 31);
    }
}