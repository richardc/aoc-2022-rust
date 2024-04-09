advent_of_code::solution!(25, 1);

fn from_snafu(input: &str) -> i64 {
    let mut accum = 0;
    for (i, b) in input.as_bytes().iter().rev().enumerate() {
        accum += 5_i64.pow(i as u32)
            * match b {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => unreachable!("not a snafu digit"),
            };
    }

    accum
}

fn to_snafu(mut input: i64) -> String {
    let mut chars = Vec::new();
    while input > 0 {
        let remainder = (input + 2) % 5;
        input = (input + 2) / 5;
        chars.push(match remainder {
            0 => '=',
            1 => '-',
            2 => '0',
            3 => '1',
            4 => '2',
            _ => unreachable!("not a snafu digit"),
        });
    }
    String::from_iter(chars.iter().rev())
}

pub fn part_one(input: &str) -> Option<String> {
    let numbers = input.lines().map(from_snafu);
    Some(to_snafu(numbers.sum()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::one(1, "1")]
    #[case::two(2, "2")]
    #[case::three(3, "1=")]
    #[case::four(4, "1-")]
    #[case::five(5, "10")]
    #[case::six(6, "11")]
    #[case::seven(7, "12")]
    #[case::eight(8, "2=")]
    #[case::nine(9, "2-")]
    #[case::ten(10, "20")]
    #[case::fifteen(15, "1=0")]
    #[case::twenty(20, "1-0")]
    #[case::twentytwentytwo(2022, "1=11-2")]
    #[case::d12345(12345, "1-0---0")]
    #[case::pi(314159265, "1121-1110-1=0")]
    fn test_snafu(#[case] decimal: i64, #[case] snafu: &str) {
        assert_eq!(from_snafu(snafu), decimal);
        assert_eq!(to_snafu(decimal), snafu);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("2=-1=0".to_string()));
    }
}
