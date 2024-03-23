advent_of_code::solution!(4);

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn new(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        let start = start.parse().unwrap();
        let end = end.parse().unwrap();
        Self { start, end }
    }

    fn contains(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|l| {
                let (a, b) = l.split_once(',').unwrap();
                let a = Range::new(a);
                let b = Range::new(b);

                a.contains(&b) || b.contains(&a)
            })
            .count(),
    )
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
