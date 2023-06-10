// https://adventofcode.com/2022/day/4

use std::ops::Range;

pub fn count_fully_containing_overlaps(assignment_pairs: Vec<AssignmentPair>) -> u32 {
    assignment_pairs
        .iter()
        .map(|pair| {
            if contains(&pair.0, &pair.1) || contains(&pair.1, &pair.0) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn count_overlaps(assignment_pairs: Vec<AssignmentPair>) -> u32 {
    assignment_pairs
        .iter()
        .map(|pair| {
            if overlaps(&pair.0, &pair.1) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn contains(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start <= b.start && b.end <= a.end
}

fn overlaps(a: &Range<u32>, b: &Range<u32>) -> bool {
    b.start < a.end && a.start <= b.start ||
    a.start < b.end && b.start <= a.start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_fully_containing_overlaps_in_example_input() {
        let assignments_pairs = parse_input(include_str!("example.txt"));

        let num_fully_containing_overlaps = count_fully_containing_overlaps(assignments_pairs);

        assert_eq!(2, num_fully_containing_overlaps);
    }

    #[test]
    fn counts_fully_containing_overlaps_in_actual_input() {
        let assignments_pairs = parse_input(include_str!("actual.txt"));

        let num_fully_containing_overlaps = count_fully_containing_overlaps(assignments_pairs);

        assert_eq!(453, num_fully_containing_overlaps);
    }

    #[test]
    fn counts_overlaps_in_example_input() {
        let assignments_pairs = parse_input(include_str!("example.txt"));

        let num_overlaps = count_overlaps(assignments_pairs);

        assert_eq!(4, num_overlaps);
    }

    #[test]
    fn counts_overlaps_in_actual_input() {
        let assignments_pairs = parse_input(include_str!("actual.txt"));

        let num_overlaps = count_overlaps(assignments_pairs);

        assert_eq!(919, num_overlaps);
    }

    fn parse_input(input: &str) -> Vec<AssignmentPair> {
        input
            .lines()
            .map(|line| line.into())
            .collect()
    }
}

pub struct AssignmentPair(Range<u32>, Range<u32>);

impl From<&str> for AssignmentPair {
    fn from(value: &str) -> Self {
        let assignments: Vec<_> = value.split(',').collect();
        AssignmentPair(
            to_range(*assignments.iter().nth(0).unwrap()),
            to_range(*assignments.iter().nth(1).unwrap())
        )
    }
}

fn to_range(value: &str) -> Range<u32> {
    let range: Vec<_> = value.split('-').collect();
    Range {
        start: range.iter().nth(0).unwrap().parse().unwrap(),
        end: range.iter().nth(1).unwrap().parse::<u32>().unwrap() + 1
    }
}