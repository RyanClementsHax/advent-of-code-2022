// https://adventofcode.com/2022/day/3

use std::collections::HashSet;

pub fn calculate_double_packed_items_priority_sum(rucksacks: Vec<Rucksack>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            let first_compartment_existence_set = rucksack.first_compartment.iter().collect::<HashSet<_>>();
            let second_compartment_existence_set = rucksack.second_compartment.iter().collect::<HashSet<_>>();
            first_compartment_existence_set
                .intersection(&second_compartment_existence_set)
                .map(|item| get_priority(**item))
                .sum::<u32>()
        })
        .sum()
}

pub fn calculate_priority_sum_of_badges(rucksack_groups: Vec<Vec<Rucksack>>) -> u32 {
    rucksack_groups
        .iter()
        .map(|group| {
            let common_items_set: HashSet<_> = group
                .iter()
                .map(|rucksack| rucksack.first_compartment
                    .iter()
                    .chain(&rucksack.second_compartment)
                    .collect::<HashSet<_>>()
                )
                .reduce(|acc, curr| acc
                    .intersection(&curr)
                    .map(|item| *item)
                    .collect()
                )
                .unwrap();
            if common_items_set.len() != 1 {
                panic!("There was a group that didn't have exactly one item in common between all of them. {:?}", group)
            }
            let common_item = *common_items_set
                .into_iter()
                .map(|item| *item)
                .collect::<Vec<_>>()
                .first()
                .unwrap();
            get_priority(common_item)
        })
        .sum()
}

fn get_priority(char: char) -> u32 {
    if char.is_ascii_lowercase() {
        // ascii a starts at 97
        (char as u32) - 96
    } else {
        // ascii A starts at 65
        (char as u32) - 64 + 26
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_the_priority_sum_of_double_packed_items_in_example_input() {
        let rucksacks = parse_input(include_str!("example.txt"));

        let sum = calculate_double_packed_items_priority_sum(rucksacks);

        assert_eq!(157, sum);
    }

    #[test]
    fn calculates_the_priority_sum_of_double_packed_items_in_actual_input() {
        let rucksacks = parse_input(include_str!("actual.txt"));

        let sum = calculate_double_packed_items_priority_sum(rucksacks);

        assert_eq!(7850, sum);
    }

    #[test]
    fn calculates_the_priority_sum_of_badges_in_actual_input() {
        let rucksacks = parse_input_grouped(include_str!("actual.txt"));

        let sum = calculate_priority_sum_of_badges(rucksacks);

        assert_eq!(2581, sum);
    }

    fn parse_input(input: &str) -> Vec<Rucksack> {
        input
            .lines()
            .map(|line| line.into())
            .collect()
    }

    fn parse_input_grouped(input: &str) -> Vec<Vec<Rucksack>> {
        input
            .lines()
            .collect::<Vec<&str>>()
            .chunks_exact(3)
            .map(|chunk| chunk
                .iter()
                .map(|line| (*line).into())
                .collect()
            )
            .collect()
    }
}

impl From<&str> for Rucksack {
    fn from(value: &str) -> Self {
        let (first_half, second_half) = value.split_at(value.len() / 2);
        Rucksack::new(
            first_half.chars().collect(), 
            second_half.chars().collect()
        )
    }
}

#[derive(Debug)]
pub struct Rucksack {
    pub first_compartment: Vec<char>,
    pub second_compartment: Vec<char>
}

impl Rucksack {
    pub fn new(first_compartment: Vec<char>, second_compartment: Vec<char>) -> Self {
        if first_compartment.len() != second_compartment.len() {
            panic!(
                "Both compartments must have the same size when making a rucksack. Found first compartment of size {} and second compartment of size {}",
                first_compartment.len(),
                second_compartment.len()
            )
        }
        Rucksack {
            first_compartment,
            second_compartment
        }
    }
}