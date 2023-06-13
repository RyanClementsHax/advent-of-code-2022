// https://adventofcode.com/2022/day/6

pub fn get_index_of_after_marker_with_window(signal: &str, window_size: usize) -> usize {
    signal
        .as_bytes()
        .windows(window_size)
        .position(|window| window
            .iter()
            .map(|char| 1_u32 << (*char as u32 - 'a' as u32))
            .fold(0, |acc, curr| acc | curr)
            .count_ones()
            == window_size as u32
        )
        .unwrap() + window_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn get_packet_index_for_example_input(expected_index: usize, signal: &str) {
        let packet_index = get_index_of_after_marker_with_window(signal, 4);

        assert_eq!(expected_index, packet_index);
    }
    
    #[test]
    fn get_packet_index_for_actual_input() {
        let signal = include_str!("actual.txt");

        let packet_index = get_index_of_after_marker_with_window(signal, 4);

        assert_eq!(1282, packet_index);
    }

    #[test_case(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(23, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(23, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(29, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(26, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn get_message_index_for_example_input(expected_index: usize, signal: &str) {
        let message_index = get_index_of_after_marker_with_window(signal, 14);

        assert_eq!(expected_index, message_index);
    }
    
    #[test]
    fn get_message_index_for_actual_input() {
        let signal = include_str!("actual.txt");

        let packet_index = get_index_of_after_marker_with_window(signal, 14);

        assert_eq!(3513, packet_index);
    }
}
