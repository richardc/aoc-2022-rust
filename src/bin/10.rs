advent_of_code::solution!(10);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let program = input
        .lines()
        .flat_map(|l| {
            if let Some(value) = l.strip_prefix("addx ") {
                let value = value.parse::<i32>().expect("to be a number");
                vec![0, value]
            } else {
                vec![0]
            }
        })
        .collect_vec();

    let mut x = 1;
    let mut total = 0;
    for (i, v) in (1..=220).zip(program.iter().cycle()) {
        if (i - 20) % 40 == 0 {
            total += i * x;
        }
        x += v;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
