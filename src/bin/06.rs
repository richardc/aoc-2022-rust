use std::collections::HashSet;

advent_of_code::solution!(6);

fn start_index(s: &str, run_length: usize) -> usize {
    s.as_bytes()
        .windows(run_length)
        .enumerate()
        .find(|(_, w)| HashSet::<u8>::from_iter(w.iter().copied()).len() == run_length)
        .unwrap()
        .0
        + run_length
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(start_index(input, 4))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(start_index(input, 14))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn start_index_test(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, start_index(input, 4))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
