use std::fs;

enum Result {
    Loss,
    Draw,
    Win,
}

impl Result {
    fn parse(input: &str) -> Result {
        match input {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Ill-formatted input: {}", input),
        }
    }
    
    fn score(&self) -> i32 {
        match self {
            Result::Loss => 0,
            Result::Draw => 3,
            Result::Win => 6,
        }
    }

    fn play(&self, other_play: Hand) -> Hand {
        match (self, other_play) {
            (Result::Loss, Hand::Rock) | (Result::Draw, Hand::Scissors) | (Result::Win, Hand::Paper) => Hand::Scissors,
            (Result::Loss, Hand::Paper) | (Result::Draw, Hand::Rock) | (Result::Win, Hand::Scissors) => Hand::Rock,
            (Result::Loss, Hand::Scissors) | (Result::Draw, Hand::Paper) | (Result::Win, Hand::Rock) => Hand::Paper,
        }
    }
}

enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn parse(input: &str) -> Hand {
        match input {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Ill-formatted input: {}", input),
        }
    }

    fn compare(&self, other: Hand) -> Result {
        match (self, other) {
            (Hand::Rock, Hand::Paper) | (Hand::Paper, Hand::Scissors) | (Hand::Scissors, Hand::Rock) => Result::Loss,
            (Hand::Rock, Hand::Rock) | (Hand::Paper, Hand::Paper) | (Hand::Scissors, Hand::Scissors) => Result::Draw,
            (Hand::Rock, Hand::Scissors) | (Hand::Paper, Hand::Rock) | (Hand::Scissors, Hand::Paper) => Result::Win,
        }
    }

    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

fn solve(file_path: &str, proper_parsing: bool) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents = contents.split("\n");

    let mut total_score = 0;
    for content in contents {
        if content == "" {
            break
        }
        let (elf_play, your_input) = (&content[0..1], &content[2..3]);
        let elf_play = Hand::parse(elf_play);
        let (your_play, result) = if !proper_parsing {
            let your_play = Hand::parse(your_input);
            let result = your_play.compare(elf_play);
            (your_play, result)
        } else {
            let result = Result::parse(your_input);
            let your_play = result.play(elf_play);
            (your_play, result)
        };
        total_score += your_play.score() + result.score();
    }
    return total_score
}

mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let file_path = "src/day2/sample.txt";
        let result = solve(file_path, false);
        assert_eq!(result, 15);
    }

    #[test]
    fn part1_input() {
        let file_path = "src/day2/input.txt";
        let result = solve(file_path, false);
        assert_eq!(result, 11449);
    }

    #[test]
    fn part2_sample() {
        let file_path = "src/day2/sample.txt";
        let result = solve(file_path, true);
        assert_eq!(result, 12);
    }

    #[test]
    fn part2_input() {
        let file_path = "src/day2/input.txt";
        let result = solve(file_path, true);
        assert_eq!(result, 13187);
    }
}
