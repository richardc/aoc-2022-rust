advent_of_code::solution!(1);

use itertools::Itertools;

fn elves(input: &str) -> Vec<u32> {
    input
        .lines()
        .group_by(|l| l.is_empty())
        .into_iter()
        .filter_map(|(empty, lines)| {
            if empty {
                None
            } else {
                Some(lines.map(|l| l.parse::<u32>().unwrap()).sum())
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    elves(input).into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(elves(input).iter().sorted().rev().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
