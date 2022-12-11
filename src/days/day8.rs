use itertools::Itertools;

use crate::err::Error;

pub struct TreeMap {
    pub heights: Vec<Vec<usize>>,
}

impl TreeMap {
    pub fn from_string_col(col: &Vec<String>) -> Result<TreeMap, Error> {
        let heights = col
            .iter()
            .map(|row| {
                row.chars()
                    .map(|inchar| {
                        inchar
                            .to_string()
                            .parse::<usize>()
                            .expect("Unable to parse char. ")
                    })
                    .collect()
            })
            .collect();

        Ok(Self { heights })
    }

    pub fn on_boundary(&self, i: usize, j: usize) -> bool {
        if i == 0
            || j == 0
            || i == self.heights.len() - 1
            || j == self
                .heights
                .first()
                .expect("Unable to find first row. ")
                .len()
                - 1
        {
            true
        } else {
            false
        }
    }

    /// Finds out whether the tree at row i and column j is visible from the
    /// outside of the map.
    pub fn is_visible(&self, i: usize, j: usize) -> bool {
        let tree = self.heights[i][j];
        let row = self.heights[i].clone();
        let col: Vec<usize> = self.heights.iter().map(|row| row[j].clone()).collect();

        if self.on_boundary(i, j)
            || tree > row[0..j].iter().fold(usize::MIN, |a, &b| a.max(b))
            || tree > row[j + 1..].iter().fold(usize::MIN, |a, &b| a.max(b))
            || tree > col[0..i].iter().fold(usize::MIN, |a, &b| a.max(b))
            || tree > col[i + 1..].iter().fold(usize::MIN, |a, &b| a.max(b))
        {
            true
        } else {
            false
        }
    }

    pub fn count_visible(&self) -> usize {
        let mut total = 0;
        for i in 0..self.heights.len() {
            for j in 0..self
                .heights
                .first()
                .expect("Unable to find first row. ")
                .len()
            {
                total += if self.is_visible(i, j) { 1 } else { 0 };
            }
        }
        total
    }

    pub fn scenic_score(&self, i: usize, j: usize) -> usize {
        let tree = self.heights[i][j];
        let row = self.heights[i].clone();
        let col: Vec<usize> = self.heights.iter().map(|row| row[j].clone()).collect();

        let left: Vec<usize> = row[0..j].into_iter().map(|v| *v).collect();
        let right: Vec<usize> = row[j + 1..].into_iter().map(|v| *v).collect();
        let up: Vec<usize> = col[0..i].into_iter().map(|v| *v).collect();
        let down: Vec<usize> = col[i + 1..].into_iter().map(|v| *v).collect();

        let left_dist = left
            .iter()
            .rev()
            .position(|t| *t >= tree)
            .map_or(left.len(), |val| val + 1);
        let right_dist = right
            .iter()
            .position(|t| *t >= tree)
            .map_or(right.len(), |val| val + 1);
        let up_dist = up
            .iter()
            .rev()
            .position(|t| *t >= tree)
            .map_or(up.len(), |val| val + 1);
        let down_dist = down
            .iter()
            .position(|t| *t >= tree)
            .map_or(down.len(), |val| val + 1);

        left_dist * right_dist * down_dist * up_dist
    }
}

#[cfg(test)]
pub mod tests {
    use itertools::Itertools;

    use super::TreeMap;
    use crate::io;
    use std::path::Path;

    #[test]
    fn day8_example() {
        let input = io::read_string_col(Path::new("data/day8/example.txt"))
            .expect("Unable to find input file. ");
        let map = TreeMap::from_string_col(&input).expect("Unable to create tree map from input. ");
        let total_visible = map.count_visible();

        assert_eq!(total_visible, 21)
    }

    #[test]
    fn day8_part1() {
        let input = io::read_string_col(Path::new("data/day8/data.txt"))
            .expect("Unable to find input file. ");
        let map = TreeMap::from_string_col(&input).expect("Unable to create tree map from input. ");
        let total_visible = map.count_visible();

        assert_eq!(total_visible, 1812)
    }

    #[test]
    fn day8_part2_example() {
        let input = io::read_string_col(Path::new("data/day8/example.txt"))
            .expect("Unable to find input file. ");
        let map = TreeMap::from_string_col(&input).expect("Unable to create tree map from input. ");

        let scores: Vec<usize> = (0..map.heights.len())
            .map(|i| {
                (0..map
                    .heights
                    .first()
                    .expect("Unable to find first row. ")
                    .len())
                    .map(|j| map.scenic_score(i, j))
                    .collect::<Vec<usize>>()
            })
            .concat();

        let max_score = scores.iter().fold(usize::MIN, |a, &b| a.max(b));
        assert_eq!(max_score, 8);
    }

    #[test]
    fn day8_part2() {
        let input = io::read_string_col(Path::new("data/day8/data.txt"))
            .expect("Unable to find input file. ");
        let map = TreeMap::from_string_col(&input).expect("Unable to create tree map from input. ");

        let scores: Vec<usize> = (0..map.heights.len())
            .map(|i| {
                (0..map
                    .heights
                    .first()
                    .expect("Unable to find first row. ")
                    .len())
                    .map(|j| map.scenic_score(i, j))
                    .collect::<Vec<usize>>()
            })
            .concat();

        let max_score = scores.iter().fold(usize::MIN, |a, &b| a.max(b));
        assert_eq!(max_score, 315495);
    }
}
