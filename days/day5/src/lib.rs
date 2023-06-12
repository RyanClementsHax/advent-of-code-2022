// https://adventofcode.com/2022/day/5

pub fn rearrange_creates(mut crate_stacks: Vec<Vec<char>>, operations: Vec<Operation>) -> Vec<Vec<char>> {
    for operation in operations {
        let source_stack = &mut crate_stacks[operation.source_stack_index];
        let start_index_to_split = source_stack.len() - operation.num_crates_to_move;
        let mut crates = source_stack.split_off(start_index_to_split);
        crates.reverse();
        crate_stacks[operation.target_stack_index].append(&mut crates)
    }
    crate_stacks
}

pub fn rearrange_creates_without_reversal(mut crate_stacks: Vec<Vec<char>>, operations: Vec<Operation>) -> Vec<Vec<char>> {
    for operation in operations {
        let source_stack = &mut crate_stacks[operation.source_stack_index];
        let start_index_to_split = source_stack.len() - operation.num_crates_to_move;
        let mut crates = source_stack.split_off(start_index_to_split);
        crate_stacks[operation.target_stack_index].append(&mut crates)
    }
    crate_stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rearranges_crates_for_example_input() {
        let (crate_stacks, operations) = parse_input(include_str!("example.txt"));

        let crate_stacks = rearrange_creates(crate_stacks, operations);

        let top_crates = get_tops_of_crate_stacks(crate_stacks);

        assert_eq!("CMZ", top_crates.iter().collect::<String>());
    }
    
    #[test]
    fn rearranges_crates_for_actual_input() {
        let (crate_stacks, operations) = parse_input(include_str!("actual.txt"));

        let crate_stacks = rearrange_creates(crate_stacks, operations);

        let top_crates = get_tops_of_crate_stacks(crate_stacks);

        assert_eq!("CVCWCRTVQ", top_crates.iter().collect::<String>());
    }

    #[test]
    fn rearranges_crates_without_reversal_for_example_input() {
        let (crate_stacks, operations) = parse_input(include_str!("example.txt"));

        let crate_stacks = rearrange_creates_without_reversal(crate_stacks, operations);

        let top_crates = get_tops_of_crate_stacks(crate_stacks);

        assert_eq!("MCD", top_crates.iter().collect::<String>());
    }

    #[test]
    fn rearranges_crates_without_reversal_for_actual_input() {
        let (crate_stacks, operations) = parse_input(include_str!("actual.txt"));

        let crate_stacks = rearrange_creates_without_reversal(crate_stacks, operations);

        let top_crates = get_tops_of_crate_stacks(crate_stacks);

        assert_eq!("CNSCZWLVT", top_crates.iter().collect::<String>());
    }

    fn get_tops_of_crate_stacks(crate_stacks: Vec<Vec<char>>) -> Vec<char> {
        crate_stacks
            .iter()
            .map(|stack| *stack.last().unwrap())
            .collect()
    }

    fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Operation>) {
        let (stacks, operations) = input.split_once("\n\n").unwrap();
        (parse_stacks(stacks), parse_operations(operations))
    }

    fn parse_stacks(input: &str) -> Vec<Vec<char>> {
        let num_stacks = input
            .lines()
            .last()
            .unwrap()
            .chars()
            .filter(|c| c.is_numeric())
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();
        let mut stacks: Vec<_> = (0..num_stacks)
            .map(|_| Vec::new())
            .collect();
        for line in input.lines().take(input.lines().count() - 1) {
            for (i, _crate) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                let crate_name = _crate[1];
                if crate_name.is_alphabetic() {
                    stacks[i].insert(0, crate_name)
                }
            }
        }
        stacks
    }

    fn parse_operations(input: &str) -> Vec<Operation> {
        input
            .lines()
            .map(|line| {
                let numbers: Vec<_> = line
                    .split_ascii_whitespace()
                    .map(|word| word.parse::<usize>())
                    .filter(|maybe_num| maybe_num.is_ok())
                    .map(|num| num.unwrap())
                    .collect();
                Operation {
                    num_crates_to_move: numbers[0],
                    source_stack_index: numbers[1] - 1,
                    target_stack_index: numbers[2] - 1
                }
            })
            .collect()
    }
}

pub struct Operation {
    num_crates_to_move: usize,
    source_stack_index: usize,
    target_stack_index: usize
}