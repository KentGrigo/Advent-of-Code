use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;

fn solve(file_path: &str, marker_length: usize) -> usize {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").collect();
    let content = contents.first().expect("You should be one and only one line of input");

    let mut marker = VecDeque::new();
    let mut marker_characters: HashSet<char> = HashSet::new();
    for (index, new_marker_character) in content.chars().enumerate() {
        if marker.len() == marker_length {
            marker.pop_front();
        }
        marker.push_back(new_marker_character);

        marker_characters.clear();
        for marker_character in marker.iter() {
            marker_characters.insert(*marker_character);
        }
        if marker_characters.len() == marker_length {
            return index + 1
        }
    }
    panic!("No marker was found!");
}

mod test {
    use super::*;

    #[test]
    fn part1_sample1() {
        let file_path = "src/day6/sample1.txt";
        let result = solve(file_path, 4);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1_sample2() {
        let file_path = "src/day6/sample2.txt";
        let result = solve(file_path, 4);
        assert_eq!(result, 5);
    }
    
    #[test]
    fn part1_sample3() {
        let file_path = "src/day6/sample3.txt";
        let result = solve(file_path, 4);
        assert_eq!(result, 6);
    }
    
    #[test]
    fn part1_sample4() {
        let file_path = "src/day6/sample4.txt";
        let result = solve(file_path, 4);
        assert_eq!(result, 10);
    }
    
    #[test]
    fn part1_sample5() {
        let file_path = "src/day6/sample5.txt";
        let result = solve(file_path, 4);
        assert_eq!(result, 11);
    }

    #[test]
    fn part1_input() {
        let file_path = "src/day6/input.txt";
        let result = solve(file_path, 4);
        assert_eq!(result, 1702);
    }

    #[test]
    fn part2_sample1() {
        let file_path = "src/day6/sample1.txt";
        let result = solve(file_path, 14);
        assert_eq!(result, 19);
    }

    #[test]
    fn part2_sample2() {
        let file_path = "src/day6/sample2.txt";
        let result = solve(file_path, 14);
        assert_eq!(result, 23);
    }
    
    #[test]
    fn part2_sample3() {
        let file_path = "src/day6/sample3.txt";
        let result = solve(file_path, 14);
        assert_eq!(result, 23);
    }
    
    #[test]
    fn part2_sample4() {
        let file_path = "src/day6/sample4.txt";
        let result = solve(file_path, 14);
        assert_eq!(result, 29);
    }
    
    #[test]
    fn part2_sample5() {
        let file_path = "src/day6/sample5.txt";
        let result = solve(file_path, 14);
        assert_eq!(result, 26);
    }

    #[test]
    fn part2_input() {
        let file_path = "src/day6/input.txt";
        let result = solve(file_path, 14);
        assert_eq!(result, 3559);
    }
}
