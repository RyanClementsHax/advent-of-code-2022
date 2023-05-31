// https://adventofcode.com/2022/day/2

pub fn calculate_score(rounds: Vec<Round>) -> i32 {
    rounds
        .iter()
        .map(|round| round.get_score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_score_of_example_input_parsed_incorrectly() {
        let rounds = parse_input_incorrectly(include_str!("example.txt"));

        let score = calculate_score(rounds);

        assert_eq!(15, score);
    }

    #[test]
    fn calculates_score_of_actual_input_parsed_incorrectly() {
        let rounds = parse_input_incorrectly(include_str!("actual.txt"));

        let score = calculate_score(rounds);

        assert_eq!(13446, score);
    }
    #[test]
    fn calculates_score_of_example_input_parsed_correctly() {
        let rounds = parse_input_correctly(include_str!("example.txt"));

        let score = calculate_score(rounds);

        assert_eq!(12, score);
    }

    #[test]
    fn calculates_score_of_actual_input_parsed_correctly() {
        let rounds = parse_input_correctly(include_str!("actual.txt"));

        let score = calculate_score(rounds);

        assert_eq!(13509, score);
    }

    fn parse_input_incorrectly(input: &str) -> Vec<Round> {
        input
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                Round {
                    opponents_choice: get_choice_naively(chars.nth(0).unwrap()),
                    your_choice: get_choice_naively(chars.nth(1).unwrap())
                }
            })
            .collect()
    }

    fn get_choice_naively(char: char) -> Choice {
        match char {
            'A' | 'X' => Choice::Rock,
            'B' | 'Y' => Choice::Paper,
            'C' | 'Z' => Choice::Scissors,
            _ => panic!("Cannot map '{}' to a choice", char)
        }
    }

    fn parse_input_correctly(input: &str) -> Vec<Round> {
        input
            .lines()
            .map(|line| {
                let mut chars = line.chars();
                let opponents_choice = get_choice_naively(chars.nth(0).unwrap());
                let your_choice = match (opponents_choice, chars.nth(1).unwrap()) {
                    (Choice::Rock, 'X') => Choice::Scissors,
                    (Choice::Rock, 'Y') => Choice::Rock,
                    (Choice::Rock, 'Z') => Choice::Paper,
                    (Choice::Paper, 'X') => Choice::Rock,
                    (Choice::Paper, 'Y') => Choice::Paper,
                    (Choice::Paper, 'Z') => Choice::Scissors,
                    (Choice::Scissors, 'X') => Choice::Paper,
                    (Choice::Scissors, 'Y') => Choice::Scissors,
                    (Choice::Scissors, 'Z') => Choice::Rock,
                    (_, char) => panic!("Could not parse input for your choice when given char {}", char)
                };
                Round {
                    opponents_choice,
                    your_choice
                }
            })
            .collect()
    }
}

pub struct Round {
    opponents_choice: Choice,
    your_choice: Choice
}

impl Round {
    fn get_score(&self) -> i32 {
        let choice_points = self.your_choice as i32;
        let result_points = self.get_result() as i32;
        choice_points + result_points
    }

    fn get_result(&self) -> RoundResult {
        match (self.your_choice, self.opponents_choice) {
            (Choice::Rock, Choice::Scissors) => RoundResult::Win,
            (Choice::Rock, Choice::Paper) => RoundResult::Lose,
            (Choice::Paper, Choice::Rock) => RoundResult::Win,
            (Choice::Paper, Choice::Scissors) => RoundResult::Lose,
            (Choice::Scissors, Choice::Paper) => RoundResult::Win,
            (Choice::Scissors, Choice::Rock) => RoundResult::Lose,
            _ => RoundResult::Draw
        }
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

pub enum RoundResult {
    Lose = 0,
    Draw = 3,
    Win = 6
}
