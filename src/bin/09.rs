use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Debug, Default)]
struct Step {
    step: usize,
    direction: Point,
}

impl Step {
    fn new(s: &str) -> Self {
        let (direction, step) = s.split_once(' ').unwrap();
        let direction = match direction {
            "U" => Point(0, 1),
            "D" => Point(0, -1),
            "L" => Point(-1, 0),
            "R" => Point(1, 0),
            _ => unreachable!("bad direction"),
        };
        let step = step.parse().unwrap();
        Self { step, direction }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Default)]
struct Rope {
    head: Point,
    tail: Point,
    tails: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        let mut rope = Self::default();
        rope.tails.insert(rope.tail);
        rope
    }

    fn move_head(&mut self, step: &Step) {
        for _ in 0..step.step {
            self.head += step.direction;
            self.drag_tail();
        }
    }

    fn drag_tail(&mut self) {
        let mut x_diff = self.head.0 - self.tail.0;
        let mut y_diff = self.head.1 - self.tail.1;
        if y_diff.abs() <= 1 && x_diff.abs() <= 1 {
            // Already touching
            return;
        }

        // We're over 2 steps away, so we slide in the lesser distance as we close the gap
        if x_diff.abs() > 2 {
            self.tail.1 = self.head.1;
            x_diff = 0;
        }

        if y_diff.abs() > 2 {
            self.tail.0 = self.head.0;
            y_diff = 0;
        }

        // slide at most one closer
        self.tail.0 += x_diff.signum();
        self.tail.1 += y_diff.signum();

        self.tails.insert(self.tail);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut rope = Rope::new();
    let steps = input.lines().map(Step::new).collect_vec();
    for s in &steps {
        rope.move_head(s);
    }
    Some(rope.tails.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
