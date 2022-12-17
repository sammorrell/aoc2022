use crate::err::Error;

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
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

#[inline]
/// Reads chunks of text, separated by an empty line.
pub fn read_text_chunks(path: &Path) -> Result<Vec<Vec<String>>, Error> {
    let file = File::open(path)?;
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .collect();

    // Now iterate over the lines and split on empty lines.
    Ok(lines
        .split(|val| val == "")
        .map(|arr| {
            arr.into_iter()
                .map(|str| str.clone())
                .collect::<Vec<String>>()
        })
        .collect())
}

#[inline]
/// Reads the contents of a file into a string.
pub fn read_string(path: &Path) -> Result<String, Error> {
    let file = File::open(path)?;
    let mut str_buf = String::new();
    let nbytes = BufReader::new(file).read_to_string(&mut str_buf)?;
    Ok(str_buf)
}
