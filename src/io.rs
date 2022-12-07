use crate::err::Error;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[inline]
pub fn read_string_col(path: &Path) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .collect();
    Ok(lines)
}

#[inline]
pub fn read_two_string_cols<const SEPARATOR: char>(
    path: &Path,
) -> Result<(Vec<String>, Vec<String>), Error> {
    let file = File::open(path)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .collect();
    let (col1, col2) = lines
        .iter()
        .map(|line| {
            let cols: Vec<&str> = line.split(SEPARATOR).map(|col| col.trim()).collect();
            assert_eq!(cols.len(), 2);
            (String::from(cols[0]), String::from(cols[1]))
        })
        .unzip();
    Ok((col1, col2))
}
