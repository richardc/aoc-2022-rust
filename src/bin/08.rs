advent_of_code::solution!(8);

use std::collections::HashSet;

use ndarray::Array2;

pub fn part_one(input: &str) -> Option<usize> {
    let dim = input.lines().count();
    let forest = Array2::from_shape_vec(
        (dim, dim),
        input
            .lines()
            .flat_map(|l| l.bytes().map(|b| b - b'0'))
            .collect(),
    )
    .unwrap();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    for (r, row) in forest.rows().into_iter().enumerate() {
        let mut highest: u8 = row[0];
        visible.insert((r, 0));
        for (c, b) in row.iter().enumerate() {
            if *b > highest {
                highest = *b;
                visible.insert((r, c));
            }
        }

        let mut highest: u8 = row[dim - 1];
        visible.insert((r, dim - 1));
        for (c, b) in row.iter().enumerate().rev() {
            if *b > highest {
                highest = *b;
                visible.insert((r, c));
            }
        }
    }

    for (c, col) in forest.columns().into_iter().enumerate() {
        let mut highest: u8 = col[0];
        visible.insert((0, c));
        for (r, b) in col.iter().enumerate() {
            if *b > highest {
                highest = *b;
                visible.insert((r, c));
            }
        }

        let mut highest: u8 = col[dim - 1];
        visible.insert((dim - 1, c));
        for (r, b) in col.iter().enumerate().rev() {
            if *b > highest {
                highest = *b;
                visible.insert((r, c));
            }
        }
    }

    Some(visible.len())
}

#[derive(Debug, Clone, Copy)]
struct Point(isize, isize);

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Point {
    fn get<'a, T>(&'a self, f: &'a Array2<T>) -> Option<&T> {
        if self.0 < 0 || self.1 < 0 {
            // Will be out of bounds if either index is < 0
            None
        } else {
            // ArrayBase.get checks for positive bounds
            f.get((self.0 as usize, self.1 as usize))
        }
    }
}

fn viewing_distance<T: std::cmp::PartialOrd<T> + std::fmt::Debug>(
    forest: &Array2<T>,
    start: (usize, usize),
    direction: Point,
) -> usize {
    let mut check = Point(start.0 as isize, start.1 as isize);
    let mut score = 0;
    let value = forest.get(start).unwrap();
    check += direction;
    while let Some(neighbour) = check.get(forest) {
        // Saw a tree.
        score += 1;

        if neighbour >= value {
            // Can't see over it though
            break;
        }

        // Keep looking
        check += direction;
    }
    score
}

fn forest(input: &str) -> Array2<u8> {
    let dim = input.lines().count();
    Array2::from_shape_vec(
        (dim, dim),
        input
            .lines()
            .flat_map(|l| l.bytes().map(|b| b - b'0'))
            .collect(),
    )
    .unwrap()
}

pub fn part_two(input: &str) -> Option<usize> {
    let forest = forest(input);

    forest
        .indexed_iter()
        .map(|(p, _)| {
            viewing_distance(&forest, p, Point(0, 1))
                * viewing_distance(&forest, p, Point(0, -1))
                * viewing_distance(&forest, p, Point(1, 0))
                * viewing_distance(&forest, p, Point(-1, 0))
        })
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_viewing_distance() {
        let forest = forest(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            viewing_distance(&forest, (1, 2), Point(0, 1)),
            2,
            "Looking right"
        );
        assert_eq!(
            viewing_distance(&forest, (1, 2), Point(0, -1)),
            1,
            "Looking left"
        );
        assert_eq!(
            viewing_distance(&forest, (1, 2), Point(1, 0)),
            2,
            "Looking down"
        );
        assert_eq!(
            viewing_distance(&forest, (1, 2), Point(-1, 0)),
            1,
            "Looking up"
        );
    }
}
