// https://adventofcode.com/2022/day/1

use std::collections::BinaryHeap;

pub fn find_calories_of_top_n_inventories(n: usize, inventories: Vec<Vec<u64>>) -> u64 {
    let max_heap = BinaryHeap::from(inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect::<Vec<u64>>()
    );
    max_heap
        .iter()
        .take(n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_most_calories_from_example_input() {
        let inventories = parse_input(include_str!("example.txt"));

        let most_calories_held_by_single_elf = find_calories_of_top_n_inventories(1, inventories);

        assert_eq!(24000, most_calories_held_by_single_elf);
    }

    #[test]
    fn finds_most_calories_from_actual_input() {
        let inventories = parse_input(include_str!("actual.txt"));

        let most_calories_held_by_single_elf = find_calories_of_top_n_inventories(1, inventories);

        assert_eq!(67622, most_calories_held_by_single_elf);
    }

    #[test]
    fn finds_sum_of_top_3_inventories_from_example_input() {
        let inventories = parse_input(include_str!("example.txt"));

        let sum_of_top_3_inventories = find_calories_of_top_n_inventories(3, inventories);

        assert_eq!(45000, sum_of_top_3_inventories);
    }

    #[test]
    fn finds_sum_of_top_3_inventories_from_actual_input() {
        let inventories = parse_input(include_str!("actual.txt"));

        let sum_of_top_3_inventories = find_calories_of_top_n_inventories(3, inventories);

        assert_eq!(201491, sum_of_top_3_inventories);
    }

    fn parse_input(input: &str) -> Vec<Vec<u64>> {
        input
            .split("\n\n")
            .map(|group| group
                .lines()
                .map(|line| line.parse().unwrap())
                .collect()
            )
            .collect()

    }
}