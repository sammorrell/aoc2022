use crate::err::Error;
use std::{
    path::Path,
};

#[cfg(test)]
mod tests {

    use std::{
        fs::File, 
        io::{BufReader, BufRead}, vec,
    };

    #[test]
    fn day1_part1() {
        let file = File::open("data/day1/data.txt").unwrap();
        let lines: Vec<String> = BufReader::new(file).lines().into_iter().map(|l| l.unwrap()).collect();
        let mut elf_calories: Vec<Vec<i32>> = vec![];
        let mut curr_elf_cals: Vec<i32> = vec![];

        for line in lines {
            match line.as_str() {
                "" => {
                    elf_calories.push(curr_elf_cals);
                    curr_elf_cals = vec![];
                },
                _ => {
                    let val: i32 =  line.parse().unwrap();
                    curr_elf_cals.push(val);
                },
            }
        }
        
        // Now I search for the largest value and print it. 
        let maxval: i32 = elf_calories.iter().map(|cals| cals.iter().sum()).max().unwrap();
        print!("{}", maxval);

        // The answer provided by AdventOfCode. 
        assert_eq!(maxval, 72478);
    }

    #[test]
    fn day1_part2() {
        let file = File::open("data/day1/data.txt").unwrap();
        let lines: Vec<String> = BufReader::new(file).lines().into_iter().map(|l| l.unwrap()).collect();
        let mut elf_calories: Vec<Vec<i32>> = vec![];
        let mut curr_elf_cals: Vec<i32> = vec![];

        for line in lines {
            match line.as_str() {
                "" => {
                    elf_calories.push(curr_elf_cals);
                    curr_elf_cals = vec![];
                },
                _ => {
                    let val: i32 =  line.parse().unwrap();
                    curr_elf_cals.push(val);
                },
            }
        }
        
        // Now I search for the largest value and print it. 
        let mut sums: Vec<i32> = elf_calories.iter().map(|cals| cals.iter().sum()).collect();
        sums.sort();
        let top3: i32 = sums.iter().rev().take(3).sum();
        println!("{}", top3);

        // The answer provided by AdventOfCode. 
        assert_eq!(top3, 210367);
    } 
}