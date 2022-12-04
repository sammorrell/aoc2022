#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::rock_paper_scissors as rps;

    #[test]
    fn day2_part1() {
        let game = rps::load_game(Path::new("data/day2/data.txt")).unwrap();
        let (p1_score, p2_score) = game.tot_scores();
        println!("{}, {}", p1_score, p2_score);
    }

    #[test]
    fn day2_part2() {
        let game = rps::load_p1_and_results(Path::new("data/day2/data.txt")).unwrap();
        let (p1_score, p2_score) = game.tot_scores();
        println!("{}, {}", p1_score, p2_score);
    }
}