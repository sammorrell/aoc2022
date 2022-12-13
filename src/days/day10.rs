#[derive(Debug, Clone)]
pub enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    pub fn from_str(input: &str) -> Command {
        let segments: Vec<&str> = input.split(" ").collect();
        match segments.first().expect("Empty string for input command. ") {
            &"noop" => Self::Noop,
            &"addx" => {
                let operand = segments[1]
                    .parse::<i32>()
                    .expect("Addx operand is not an integer. ");
                Self::Addx(operand)
            }
            _ => panic!("Unknown command. "),
        }
    }

    pub fn cycles(&self) -> i32 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }

    pub fn process(&self, input: &i32) -> i32 {
        match self {
            Self::Noop => input.clone(),
            Self::Addx(ref val) => input + val,
        }
    }
}

pub fn value_at_cycle(commands: &Vec<Command>, target_cycle: i32) -> i32 {
    let mut cycles = 0;
    let mut x = 1;

    for com in commands {
        cycles += com.cycles();
        if cycles >= target_cycle {
            break;
        }
        x = com.process(&x)
    }
    x
}

pub fn total_cycles(commands: &Vec<Command>) -> i32 {
    commands.iter().map(|com| com.cycles()).sum()
}

pub fn render_image(commands: &Vec<Command>) -> String {
    let width: i32 = 40;
    let mut output = String::new();
    for cyc in 1..total_cycles(commands) {
        let val = value_at_cycle(commands, cyc as i32);
        let curr_pixel = (cyc as i32 - 1) % width;
        // Check that the current pixel is within 1 pixel of the cursor position. 
        if (curr_pixel - val).abs() < 2 {
            output.push('#');
        } else {
            output.push('.');
        }

        // If we are at the end of a line, we want to output a newline. 
        if curr_pixel == width - 1 {
            output.push('\n');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{render_image, value_at_cycle, Command};
    use crate::io;
    use std::path::Path;

    #[test]
    pub fn day10_example() {
        let input_vec: Vec<String> =
            io::read_string_col(Path::new("data/day10/example.txt")).expect("No input found. ");
        let commands: Vec<Command> = input_vec
            .iter()
            .map(|string| Command::from_str(string.as_str()))
            .collect();
        let x: i32 = 1;

        let c20 = value_at_cycle(&commands, 20);
        assert_eq!(c20 * 20, 420);
        let c60 = value_at_cycle(&commands, 60);
        assert_eq!(c60 * 60, 1140);
        let c100 = value_at_cycle(&commands, 100);
        assert_eq!(c100 * 100, 1800);
        let c140 = value_at_cycle(&commands, 140);
        assert_eq!(c140 * 140, 2940);
        let c180 = value_at_cycle(&commands, 180);
        assert_eq!(c180 * 180, 2880);
        let c220 = value_at_cycle(&commands, 220);
        assert_eq!(c220 * 220, 3960);

        assert_eq!(
            c20 * 20 + c60 * 60 + c100 * 100 + c140 * 140 + c180 * 180 + c220 * 220,
            13140
        );
    }

    #[test]
    pub fn day10_part1() {
        let input_vec: Vec<String> =
            io::read_string_col(Path::new("data/day10/data.txt")).expect("No input found. ");
        let commands: Vec<Command> = input_vec
            .iter()
            .map(|string| Command::from_str(string.as_str()))
            .collect();
        let x: i32 = 1;

        let c20 = value_at_cycle(&commands, 20);
        let c60 = value_at_cycle(&commands, 60);
        let c100 = value_at_cycle(&commands, 100);
        let c140 = value_at_cycle(&commands, 140);
        let c180 = value_at_cycle(&commands, 180);
        let c220 = value_at_cycle(&commands, 220);

        assert_eq!(
            c20 * 20 + c60 * 60 + c100 * 100 + c140 * 140 + c180 * 180 + c220 * 220,
            13140
        );
    }

    #[test]
    pub fn day10_part2() {
        let input_vec: Vec<String> =
            io::read_string_col(Path::new("data/day10/data.txt")).expect("No input found. ");
        let commands: Vec<Command> = input_vec
            .iter()
            .map(|string| Command::from_str(string.as_str()))
            .collect();
        let out = render_image(&commands);
        println!("{}", out);

        // Nothing to really test here, because the output is to the command line.
        // However the correct answer is EALGULPG.
    }
}
