use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Rock,
    Sand,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point(i32, i32);

impl Point {
    fn new(s: &str) -> Self {
        let (x, y) = s.split_once(',').expect("comma");
        let x = x.parse().expect("number for x");
        let y = y.parse().expect("number for y");
        Self(x, y)
    }
}

#[derive(Debug)]
struct Well {
    contents: HashMap<Point, Cell>,
    lowest_block: i32,
    hard_floor: Option<i32>,
}

impl Well {
    fn new(s: &str) -> Self {
        let mut contents = HashMap::new();
        let mut lowest_block = 0;
        for line in s.lines() {
            for (start, end) in line.split(" -> ").map(Point::new).tuple_windows() {
                let (x1, x2) = if start.0 > end.0 {
                    (end.0, start.0)
                } else {
                    (start.0, end.0)
                };
                let (y1, y2) = if start.1 > end.1 {
                    (end.1, start.1)
                } else {
                    (start.1, end.1)
                };
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        contents.insert(Point(x, y), Cell::Rock);
                        lowest_block = lowest_block.max(y);
                    }
                }
            }
        }
        Self {
            contents,
            lowest_block,
            hard_floor: None,
        }
    }

    fn drop(&mut self, start: Point) -> bool {
        if start.1 > self.lowest_block {
            // In freefall
            return false;
        }

        if self.contents.contains_key(&start) {
            return false;
        }

        if let Some(floor) = self.hard_floor {
            if start.1 + 1 == floor {
                self.contents.insert(start, Cell::Sand);
                return true;
            }
        }

        let below = Point(start.0, start.1 + 1);
        let left = Point(start.0 - 1, start.1 + 1);
        let right = Point(start.0 + 1, start.1 + 1);
        if self.contents.contains_key(&below) {
            if self.contents.contains_key(&left) {
                if self.contents.contains_key(&right) {
                    // settle
                    self.contents.insert(start, Cell::Sand);
                    true
                } else {
                    self.drop(right)
                }
            } else {
                self.drop(left)
            }
        } else {
            self.drop(below)
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut well = Well::new(input);
    while well.drop(Point(500, 0)) {}
    Some(well.contents.values().filter(|&v| *v == Cell::Sand).count())
}

impl Well {
    fn add_floor(&mut self) {
        let y = self.lowest_block + 2;
        self.lowest_block += 2;
        self.hard_floor = Some(y);
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut well = Well::new(input);
    well.add_floor();
    while well.drop(Point(500, 0)) {}
    Some(well.contents.values().filter(|&v| *v == Cell::Sand).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(93));
    }
}
