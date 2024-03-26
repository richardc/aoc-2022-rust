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
    segments: Vec<Point>,
    tails: HashSet<Point>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            segments: Vec::from_iter(std::iter::repeat(Point::default()).take(length + 1)),
            tails: HashSet::from_iter([Point::default()]),
        }
    }

    fn move_head(&mut self, step: &Step) {
        for _ in 0..step.step {
            // Move the head
            self.segments[0] += step.direction;

            // For each pair, everyone gets to be the boss once
            for i in 0..(self.segments.len() - 1) {
                Self::drag_segment(self.segments[i], &mut self.segments[i + 1]);
            }

            // And we're just after the last one
            self.tails.insert(self.segments[self.segments.len() - 1]);
        }
    }

    fn drag_segment(head: Point, tail: &mut Point) {
        let mut x_diff = head.0 - tail.0;
        let mut y_diff = head.1 - tail.1;
        if y_diff.abs() <= 1 && x_diff.abs() <= 1 {
            // Already touching
            return;
        }

        // We're over 2 steps away, so we slide in the lesser distance as we close the gap
        if x_diff.abs() > 2 {
            tail.1 = head.1;
            x_diff = 0;
        }

        if y_diff.abs() > 2 {
            tail.0 = head.0;
            y_diff = 0;
        }

        // slide at most one closer
        tail.0 += x_diff.signum();
        tail.1 += y_diff.signum();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut rope = Rope::new(1);
    let steps = input.lines().map(Step::new).collect_vec();
    for s in &steps {
        rope.move_head(s);
    }
    Some(rope.tails.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut rope = Rope::new(9);
    let steps = input.lines().map(Step::new).collect_vec();
    for s in &steps {
        rope.move_head(s);
    }
    Some(rope.tails.len())
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
    fn test_part_two_original() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(36));
    }
}
