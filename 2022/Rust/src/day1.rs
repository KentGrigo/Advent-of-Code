use std::fs;

fn solve(file_path: &str, number_of_top_elves: usize) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents = contents.split("\n");

    let mut max_calories_by_elves = Vec::new();
    let mut calories = 0;
    for content in contents {
        if content.is_empty() {
            max_calories_by_elves.push(calories);
            calories = 0;
        } else {
            calories += content.parse::<i32>().unwrap();
        }
    }
    max_calories_by_elves.sort();
    let top_calories = max_calories_by_elves
        .iter()
        .rev()
        .take(number_of_top_elves)
        .sum();
    return top_calories
}

mod test {
    use super::*;

    #[test]
    fn sample() {
        let file_path = "src/day1/sample.txt";
        let result = solve(file_path, 1);
        assert_eq!(result, 24000);
    }

    #[test]
    fn part1() {
        let file_path = "src/day1/input.txt";
        let result = solve(file_path, 1);
        assert_eq!(result, 70509);
    }

    #[test]
    fn part2() {
        let file_path = "src/day1/input.txt";
        let result = solve(file_path, 3);
        assert_eq!(result, 208567);
    }
}
