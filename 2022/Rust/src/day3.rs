use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn solve_part1(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").collect();

    let mut sum_of_priorities = 0;
    for rucksack in contents {
        if rucksack.is_empty() {
            break
        }
        let overlapping_item = personal_overlapping_item(rucksack);
        let priority = item_to_priority(overlapping_item);
        sum_of_priorities += priority;
    }
    return sum_of_priorities
}

fn solve_part2(file_path: &str) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = contents.split("\n").collect();

    let mut sum_of_group_priorities = 0;
    for (rucksack1, rucksack2, rucksack3) in contents.iter().tuples() {
        let overlapping_group_item = group_overlapping_item(rucksack1, rucksack2, rucksack3);
        let priority = item_to_priority(overlapping_group_item);
        sum_of_group_priorities += priority;
    }
    return sum_of_group_priorities
}

fn personal_overlapping_item(rucksack: &str) -> char {
    let (left_compartment, right_compartment) = rucksack.split_at(rucksack.len() / 2);
    let left_items: HashSet<char> = left_compartment.chars().collect();
    let right_items: HashSet<char> = right_compartment.chars().collect();
    let overlapping_item = left_items.intersection(&right_items)
        .next()
        .expect("Expected one and only one overlapping item, but found none.");
    return *overlapping_item
}

fn group_overlapping_item(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> char {
    let items1: HashSet<char> = rucksack1.chars().collect();
    let items2: HashSet<char> = rucksack2.chars().collect();
    let items3: HashSet<char> = rucksack3.chars().collect();
    let mut overlapping_team_items: HashSet<char> = items1;
    overlapping_team_items = overlapping_team_items.intersection(&items2)
        .map(|item| *item)
        .collect();
    overlapping_team_items = overlapping_team_items.intersection(&items3)
        .map(|item| *item)
        .collect();
    let overlapping_team_item = overlapping_team_items
        .iter()
        .next()
        .expect("Expected one and only one overlapping item, but found none.");
    return *overlapping_team_item
}

fn item_to_priority(item: char) -> i32 {
    let mut priority = item as i32 - 96;
    if priority < 0 {
        priority += 58;
    }
    return priority
}

mod test {
    use super::*;

    #[test]
    fn part1_sample() {
        let file_path = "src/day3/sample.txt";
        let result = solve_part1(file_path);
        assert_eq!(result, 157);
    }

    #[test]
    fn part1_input() {
        let file_path = "src/day3/input.txt";
        let result = solve_part1(file_path);
        assert_eq!(result, 8139);
    }

    #[test]
    fn part2_sample() {
        let file_path = "src/day3/sample.txt";
        let result = solve_part2(file_path);
        assert_eq!(result, 70);
    }

    #[test]
    fn part2_input() {
        let file_path = "src/day3/input.txt";
        let result = solve_part2(file_path);
        assert_eq!(result, 2668);
    }
}
