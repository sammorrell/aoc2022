use std::ops::Range;

pub fn contains(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    r1.start <= r2.start && r1.end >= r2.end
}

pub fn overlaps(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    (r1.start >= r2.start && r1.start <= r2.end) || (r1.end >= r2.start && r1.end <= r2.end)
}

#[cfg(test)]
mod tests {
    use crate::io::read_two_string_cols;
    use itertools::*;
    use std::{ops::Range, path::Path};

    use super::{contains, overlaps};

    #[test]
    fn day4_example() {
        let (col1, col2) = read_two_string_cols::<','>(Path::new("data/day4/example.txt")).unwrap();

        let range1: Vec<Range<usize>> = col1
            .iter()
            .map(|str| {
                let components: (usize, usize) = str
                    .split("-")
                    .map(|comp| comp.parse::<usize>().unwrap())
                    .into_iter()
                    .collect_tuple()
                    .unwrap();
                components.0..components.1
            })
            .collect();

        let range2: Vec<Range<usize>> = col2
            .iter()
            .map(|str| {
                let components: (usize, usize) = str
                    .split("-")
                    .map(|comp| comp.parse::<usize>().unwrap())
                    .into_iter()
                    .collect_tuple()
                    .unwrap();
                components.0..components.1
            })
            .collect();

        let num_contains: usize = range1
            .into_iter()
            .zip(range2)
            .map(|(r1, r2)| {
                if contains(&r1, &r2) || contains(&r2, &r1) {
                    1
                } else {
                    0
                }
            })
            .sum();

        // The answer provided by the example.
        assert_eq!(num_contains, 2);
    }

    #[test]
    fn day4_part1() {
        let (col1, col2) = read_two_string_cols::<','>(Path::new("data/day4/data.txt")).unwrap();

        let range1: Vec<Range<usize>> = col1
            .iter()
            .map(|str| {
                let components: (usize, usize) = str
                    .split("-")
                    .map(|comp| comp.parse::<usize>().unwrap())
                    .into_iter()
                    .collect_tuple()
                    .unwrap();
                components.0..components.1
            })
            .collect();

        let range2: Vec<Range<usize>> = col2
            .iter()
            .map(|str| {
                let components: (usize, usize) = str
                    .split("-")
                    .map(|comp| comp.parse::<usize>().unwrap())
                    .into_iter()
                    .collect_tuple()
                    .unwrap();
                components.0..components.1
            })
            .collect();

        let num_contains: usize = range1
            .into_iter()
            .zip(range2)
            .map(|(r1, r2)| {
                if contains(&r1, &r2) || contains(&r2, &r1) {
                    1
                } else {
                    0
                }
            })
            .sum();

        //
        assert_eq!(num_contains, 530);
    }

    #[test]
    fn day4_part2() {
        let (col1, col2) = read_two_string_cols::<','>(Path::new("data/day4/data.txt")).unwrap();

        let range1: Vec<Range<usize>> = col1
            .iter()
            .map(|str| {
                let components: (usize, usize) = str
                    .split("-")
                    .map(|comp| comp.parse::<usize>().unwrap())
                    .into_iter()
                    .collect_tuple()
                    .unwrap();
                components.0..components.1
            })
            .collect();

        let range2: Vec<Range<usize>> = col2
            .iter()
            .map(|str| {
                let components: (usize, usize) = str
                    .split("-")
                    .map(|comp| comp.parse::<usize>().unwrap())
                    .into_iter()
                    .collect_tuple()
                    .unwrap();
                components.0..components.1
            })
            .collect();

        let num_contains: usize = range1
            .into_iter()
            .zip(range2)
            .map(|(r1, r2)| {
                if overlaps(&r1, &r2) || overlaps(&r2, &r1) {
                    1
                } else {
                    0
                }
            })
            .sum();

        // The answer provided by AOC for part 2.
        assert_eq!(num_contains, 903);
    }
}
