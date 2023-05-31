// https://adventofcode.com/2022/day/25

pub fn sum_snafus(snafus: Vec<SNAFU>) -> SNAFU {
    snafus
        .iter()
        .map(|snafu| snafu.decimal_num)
        .sum::<i64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_example_input_to_snafu_sum() {
        let snafus = parse_input(include_str!("example.txt"));

        let snafu_num_of_sum = sum_snafus(snafus);

        assert_eq!("2=-1=0", snafu_num_of_sum.snafu_num);
    }

    #[test]
    fn converts_actual_input_to_snafu_sum() {
        let snafus = parse_input(include_str!("actual.txt"));

        let snafu_num_of_sum = sum_snafus(snafus);

        assert_eq!("2---1010-0=1220-=010", snafu_num_of_sum.snafu_num);
    }

    fn parse_input(input: &str) -> Vec<SNAFU> {
        input
            .lines()
            .map(|line| SNAFU::from(line))
            .collect()
    }
}

#[derive(Debug)]
pub struct SNAFU {
    pub snafu_num: String,
    pub decimal_num: i64
}

impl From<i64> for SNAFU {
    // Reimplementation of
    // https://github.com/jswalden/adventofcode2022/blob/main/day-25/src/main.rs
    fn from(value: i64) -> Self {
        let num_digits = count_snafu_digits(value);

        let mut snafu_num = String::new();
        
        let mut remaining = value;
        let mut min_snafu: i64 = (0..num_digits).fold(0, |sum, _| sum * 5 - 2);
        let mut max_snafu: i64 = (0..num_digits).fold(0, |sum, _| sum * 5 + 2);
        let mut digit_value = 5_i64.pow(num_digits as u32 - 1);
        for _ in 0..num_digits {
            let digit = (remaining - min_snafu) / digit_value;
            let snafu_digit = match digit - 2 {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => panic!("Tried to serialize digit {} for {}", digit, value)
            };
            snafu_num.push(snafu_digit);
    
            remaining -= (digit - 2) * digit_value;
            min_snafu = (min_snafu + 2) / 5;
            max_snafu = (max_snafu + 2) / 5;
            digit_value /= 5;
        }
        SNAFU {
            snafu_num,
            decimal_num: value
        }
    }
}

fn count_snafu_digits(value: i64) -> i64 {
    let mut digit_count = 1;
    let mut upper_limit = 2;

    while value > upper_limit {
        digit_count += 1;
        upper_limit = upper_limit * 5 + 2;
    }

    digit_count
}

impl From<&SNAFU> for i64 {
    fn from(value: &SNAFU) -> Self {
        value.decimal_num
    }
}

impl From<SNAFU> for i64 {
    fn from(value: SNAFU) -> Self {
        i64::from(&value)
    }
}

impl From<String> for SNAFU {
    fn from(value: String) -> Self {
        SNAFU {
            decimal_num: value
                .chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    let coeficient = match c {
                        '2' => 2,
                        '1' => 1,
                        '0' => 0,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!("Unknown character in snafu representation of string {} in {:?}", c, value)
                    };
                    coeficient * 5_i64.pow(i.try_into().unwrap())
                })
                .sum(),
            snafu_num: value,
        }
    }
}

impl From<&str> for SNAFU {
    fn from(value: &str) -> Self {
        String::from(value).into()
    }
}
