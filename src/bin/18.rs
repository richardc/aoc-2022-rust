advent_of_code::solution!(18);

use itertools::{iproduct, Itertools};

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(s: &str) -> Self {
        let (x, y, z) = sscanf::sscanf!(s, "{i32},{i32},{i32}").expect("cube");
        Self { x, y, z }
    }

    fn adjacent(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y && self.z.abs_diff(other.z) == 1 {
            return true;
        }
        if self.x == other.x && self.z == other.z && self.y.abs_diff(other.y) == 1 {
            return true;
        }
        if self.z == other.z && self.y == other.y && self.x.abs_diff(other.x) == 1 {
            return true;
        }
        false
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let cubes = input.lines().map(Cube::new).collect_vec();
    let mut count = cubes.len() * 6;
    for (a, b) in iproduct!(cubes.clone(), cubes.clone()) {
        if a.adjacent(&b) {
            count -= 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
