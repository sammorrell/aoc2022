use std::{collections::VecDeque, vec};

#[derive(Debug, Clone)]
pub enum Token {
    Command { cmd: String, args: Vec<String> },
    DirectoryItem { name: String },
    FileItem { name: String, size: usize },
}

#[derive(Debug)]
pub enum TreeItem {
    Directory {
        name: String,
        children: Vec<TreeItem>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl TreeItem {
    pub fn from_tokens(name: String, tokens: &mut VecDeque<Token>) -> TreeItem {
        let mut current_item = TreeItem::Directory {
            name,
            children: vec![],
        };
        current_item.process_tokens(tokens);
        current_item
    }

    pub fn size(&self) -> usize {
        match self {
            Self::Directory { name: _, ref children } => children.iter().map(Self::size).sum(),
            Self::File { name: _, ref size } => size.clone(),
        }
    }

    pub fn process_tokens(&mut self, tokens: &mut VecDeque<Token>) {
        while let Some(tok) = tokens.pop_front() {
            match tok {
                Token::FileItem { ref name, ref size } => match self {
                    TreeItem::Directory { name: _, children } => children.push(Self::File {
                        name: name.clone(),
                        size: size.clone(),
                    }),
                    _ => panic!(),
                },
                Token::DirectoryItem { ref name } => match self {
                    TreeItem::Directory { name: _, children } => children.push(Self::Directory {
                        name: name.clone(),
                        children: vec![],
                    }),
                    _ => panic!(),
                },
                Token::Command { ref cmd, ref args } => match cmd.as_str() {
                    "cd" => {
                        let target_dir = args.first().expect("expect arg after cd. ");
                        match target_dir.as_str() {
                            ".." => break,
                            _ => match self.find_item(target_dir) {
                                Some(dir) => dir.process_tokens(tokens),
                                None => panic!("{} not found in directory", target_dir),
                            },
                        }
                    }
                    _ => {}
                },
            }
        }
    }

    pub fn find_item(&mut self, search_name: &String) -> Option<&mut TreeItem> {
        match self {
            Self::File { name: _, size: _ } => None,
            Self::Directory { name: _, children } => children.iter_mut().find(|item| match *item {
                Self::File { ref name, size: _ } => name == search_name,
                Self::Directory {
                    ref name,
                    children: _,
                } => name == search_name,
            }),
        }
    }

    pub fn find_total_size(&self, target_size: usize) -> Vec<&TreeItem> {
        match self {
            Self::Directory { name: _, ref children } => {
                let this_dir = if self.size() <= target_size { vec![self] } else { vec![] };
                [this_dir, children.iter().map(|item| item.find_total_size(target_size)).flatten().collect::<Vec<&TreeItem>>()].concat()
            },
            Self::File { name: _, size: _ } => vec![],
        }
    }
}

fn tokenise_vec(str_vec: &Vec<String>) -> Vec<Token> {
    str_vec
        .iter()
        .map(|line| {
            let segments: Vec<&str> = line.split(" ").collect();

            match *segments.first().expect("Empty line. ") {
                "$" => Token::Command {
                    cmd: segments[1].to_string(),
                    args: segments
                        .into_iter()
                        .skip(2)
                        .map(str::to_string)
                        .collect::<Vec<String>>(),
                },
                "dir" => Token::DirectoryItem {
                    name: segments[1].to_string(),
                },
                num if num.trim().parse::<usize>().is_ok() => Token::FileItem {
                    name: segments[1].to_string(),
                    size: num
                        .trim()
                        .parse::<usize>()
                        .expect("Number is not a number. "),
                },
                _ => panic!("Unexpected state reached when parsing tokens. "),
            }
        })
        .collect()
}

fn parse_tree_from_tokens(tokens: &Vec<Token>) -> TreeItem {
    let mut tok_vec = VecDeque::from(tokens.clone());

    // Check that we have the root node in the first position.
    let test = tok_vec.pop_front().expect("No tokens in input. ");
    match test {
        Token::Command { ref cmd, ref args } => {
            assert_eq!(cmd, "cd");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], "/");

            TreeItem::from_tokens(args[0].clone(), &mut tok_vec)
        }
        _ => panic!("First item in input is not a command. "),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_tree_from_tokens, tokenise_vec, TreeItem};
    use crate::io::read_string_col;
    use std::path::Path;

    #[test]
    pub fn day7_example() {
        let lines = read_string_col(Path::new("data/day7/example.txt")).expect("Empty file. ");
        let tokens = tokenise_vec(&lines);
        let tree = parse_tree_from_tokens(&tokens);
        
        // Check that the total size is correct. 
        assert_eq!(tree.size(), 48381165);

        let found_dirs = tree.find_total_size(100_000);
        let found_dirs_sum: usize = found_dirs.into_iter().map(TreeItem::size).sum();
        assert_eq!(found_dirs_sum, 94853 + 584);
    }

    #[test]
    pub fn day7_part1() {
        let lines = read_string_col(Path::new("data/day7/data.txt")).expect("Empty file. ");
        let tokens = tokenise_vec(&lines);
        let tree = parse_tree_from_tokens(&tokens);

        let found_dirs = tree.find_total_size(100_000);
        let found_dirs_sum: usize = found_dirs.into_iter().map(TreeItem::size).sum();
        assert_eq!(found_dirs_sum, 584);
    }
}
