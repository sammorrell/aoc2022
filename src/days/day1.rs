use crate::err::Error;
use std::{
    path::Path,
};

#[derive(Debug, Clone)]
struct ElfCalories {
    item_calories: Vec<f64>,
}

impl ElfCalories {
    #[must_use]
    #[inline]
    pub fn new_with_items(items: Vec<f64>) -> Self {
        Self { item_calories: items }
    }

    pub fn from_file(path: &Path) -> Result<Vec<ElfCalories>, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn day1_part1() {
        
    }
}