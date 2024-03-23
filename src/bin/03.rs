advent_of_code::solution!(3);

use std::collections::HashSet;

fn priority(b: u8) -> u8 {
    match b {
        b'a'..=b'z' => b - b'a' + 1,
        b'A'..=b'Z' => b - b'A' + 27,
        _ => unreachable!("priority of {}", b as char),
    }
}

fn pack_priority(s: &str) -> u32 {
    let mid = s.len() / 2;
    let (left, right) = (&s[..mid], &s[mid..]);
    let left: HashSet<u8> = HashSet::from_iter(left.bytes());
    let right: HashSet<u8> = HashSet::from_iter(right.bytes());
    left.intersection(&right).map(|&b| priority(b) as u32).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(pack_priority).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority(b'a'), 1);
        assert_eq!(priority(b'A'), 27);
    }

    #[test]
    fn test_pack_priority() {
        assert_eq!(pack_priority("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
