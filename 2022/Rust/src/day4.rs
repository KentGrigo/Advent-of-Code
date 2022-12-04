use std::{fs};

fn solve(file_path: &str) -> (i32, i32) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").collect();

    let mut number_of_fully_containments = 0;
    let mut number_of_overlaps = 0;
    for content in contents {
        if content == "" {
            break
        }

        let (elf1, elf2) = split_at(content.to_owned(), ',');
        let (elf1_min, elf1_max) = interval(elf1);
        let (elf2_min, elf2_max) = interval(elf2);

        let elf1_fully_contains_elf2 = fully_contains(elf1_min, elf1_max, elf2_min, elf2_max);
        let elf2_fully_contains_elf1 = fully_contains(elf2_min, elf2_max, elf1_min, elf1_max);
        if elf1_fully_contains_elf2 || elf2_fully_contains_elf1 {
            number_of_fully_containments += 1;
        }

        let elf1_overlaps_elf2 = overlaps(elf1_min, elf1_max, elf2_min, elf2_max);
        let elf2_overlaps_elf1 = overlaps(elf2_min, elf2_max, elf1_min, elf1_max);
        if elf1_overlaps_elf2 || elf2_overlaps_elf1 {
            number_of_overlaps += 1;
        }
    }
    return (number_of_fully_containments, number_of_overlaps)
}

fn split_at(text: String, separator: char) -> (String, String) {
    let index_of_separator = text.find(separator)
        .expect(&format!("Ill-formatted input, must contain `{}`", separator));
    return (text[..index_of_separator].to_owned(), text[index_of_separator+1..].to_owned())
}

fn interval(elf: String) -> (i32, i32) {
    let (elf_min, elf_max) = split_at(elf, '-');
    return (elf_min.parse::<i32>().unwrap(), elf_max.parse::<i32>().unwrap());
}

fn fully_contains(elf1_min: i32, elf1_max: i32, elf2_min: i32, elf2_max: i32) -> bool {
    return elf1_min <= elf2_min && elf2_max <= elf1_max
}

fn overlaps(elf1_min: i32, elf1_max: i32, elf2_min: i32, elf2_max: i32) -> bool {
    return elf1_min <= elf2_min && elf2_min <= elf1_max || elf1_min <= elf2_max && elf2_max <= elf1_max
}

mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let file_path = "src/day4/sample.txt";
        let (result, _) = solve(file_path);
        assert_eq!(result, 2);
    }

    #[test]
    fn part1_input() {
        let file_path = "src/day4/input.txt";
        let (result, _) = solve(file_path);
        assert_eq!(result, 464);
    }

    #[test]
    fn part2_sample() {
        let file_path = "src/day4/sample.txt";
        let (_, result) = solve(file_path);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_input() {
        let file_path = "src/day4/input.txt";
        let (_, result) = solve(file_path);
        assert_eq!(result, 770);
    }
}
