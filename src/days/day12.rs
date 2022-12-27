// This one I got stuck on, as I have not done decision maths for quite a while.
// However I looked through the other solutions and was inspired by:
// https://github.com/frjonsen/aoc2022/blob/master/day12/part2/src/main.rs#L211
//
// Also a big thank you to Red Blob Games for a great write-up of pathfinding algorithms:
// https://www.redblobgames.com/pathfinding/a-star/introduction.html

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

    pub fn coords_at_elevation(&self, elev: u8) -> Vec<Coord> {
        let mut found_coords = vec![];
        for (i, row) in self.data.iter().enumerate() {
            for (j, node) in row.iter().enumerate() {
                if <i32 as From<GridNode>>::from(*node) as u8 == elev {
                    found_coords.push(Coord(i, j));
                }
            }
        }
        found_coords
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
    let height_diff = grid.data()[next_coord.0][next_coord.1] - grid.data()[curr_coord.0][curr_coord.1];
    height_diff <= 1
}

pub fn find_path_with_a_star(grid: &Grid, start: &Coord, end: Vec<Coord>) -> Option<Vec<Coord>> {
    let mut frontier: Vec<Rc<Node>> = vec![];
    let mut came_from: Vec<Rc<Node>> = vec![];
    let mut visited: Vec<Coord> = vec![];

    // Push the starting point onto the closed list to begin. 
    let start_node = Node::new(None, start);
    frontier.push(Rc::new(start_node));

    while !frontier.is_empty() {
        // Get the next highest priority in the queue of frontier nodes. 
        let (next_idx, _) = frontier
            .iter()
            .enumerate()
            .min_by_key(|(_, node)| node.f() )
            .unwrap();

        // Remove current node from open list. 
        let curr_node = {
            let curr_node = frontier.swap_remove(next_idx);
            came_from.push(curr_node);
            came_from.last().unwrap()
        };

        // Test to see if we are done. 
        if end.contains(&curr_node.coord) {
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
                        // Heuristic is the distance to the goal. 
                        h: end.first().unwrap().0.abs_diff(neigh.0) +  end.first().unwrap().1.abs_diff(neigh.1),
                        coord: neigh,
                        // Cost is the distance from the original starting point.
                        // As this is a single step each time, we update accordingly. 
                        g: curr_node.g + 1,
                    }
                )
            });

        for neighbour in neighbours {
            // Check to see if we have already visisted this neighbour in our journey. 
            if visited.contains(&neighbour.coord) { continue; }
            visited.push(neighbour.coord.clone());
            
            for point in frontier.iter() {
                // Check to see if there is a lower cost alternative alreadt in the frontier. 
                if point.coord == neighbour.coord && neighbour.g > point.g {
                    continue;
                }
            }

            frontier.push(neighbour);
        }
    };

    // If we reach this point, something has gone wrong. 
    None
}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;

    use crate::days::day12::find_path_with_a_star;
    use super::Grid;

    #[test]
    pub fn day12_example() {
        let grid = Grid::from_string(include_str!("../../data/day12/example.txt"));
        let start_coord = grid.start_coord();
        let end_coord = vec![grid.end_coord()];

        let route = find_path_with_a_star(&grid, &start_coord, end_coord).unwrap();
        assert_eq!(route.len(), 31);
    }

    #[test]
    pub fn day12_part1() {
        let grid = Grid::from_string(include_str!("../../data/day12/data.txt"));
        let start_coord = grid.start_coord();
        let end_coord = vec![grid.end_coord()];

        let route = find_path_with_a_star(&grid, &start_coord, end_coord).unwrap();
        assert_eq!(route.len(), 517);
    }

    #[test]
    pub fn day12_example2() {
        let grid = Grid::from_string(include_str!("../../data/day12/example.txt"));
        let start_coords = grid.coords_at_elevation(0);
        let end_coord = vec![grid.end_coord()];

        let route_lengths: Vec<usize> = start_coords.iter().map(|start| {
            match find_path_with_a_star(&grid, &start, end_coord.clone()) {
                Some(vec) => vec.len(),
                None => usize::MAX
            }
        }).collect();
        let shortest_route = route_lengths.into_iter().fold(usize::MAX, |accum, val| accum.min(val) );

        assert_eq!(shortest_route, 29);
    }

    // This appears to be a working solution for it, however it takes too long to actually run.
    // I think there is an issue with the A* implementation, as the  algorithm took about 46 minutes to run for a single. 
    // So, some refinements to make. In particular, flip this around and adopt a Dijkstra's approach to this in order to
    // find the closest route to all of the available end points. 
    #[test]
    #[ignore]
    pub fn day12_part2() {
        let grid = Grid::from_string(include_str!("../../data/day12/data.txt"));
        let start_coords = grid.coords_at_elevation(0);
        let end_coord = vec![grid.end_coord()];

        let route_lengths: Vec<usize> = start_coords.iter().map(|start| {
            match find_path_with_a_star(&grid, &start, end_coord.clone()) {
                Some(vec) => vec.len(),
                None => usize::MAX
            }
        }).collect();
        let shortest_route = route_lengths.into_iter().fold(usize::MAX, |accum, val| accum.min(val) );

        assert_eq!(shortest_route, 29);
    }
}