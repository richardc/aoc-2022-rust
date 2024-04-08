use std::collections::{HashMap, HashSet};

use itertools::Itertools;
advent_of_code::solution!(23);

struct Elves {
    step: usize,
    locations: HashSet<(isize, isize)>,
}

impl std::fmt::Display for Elves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (&min_row, &max_row) = self
            .locations
            .iter()
            .map(|(r, _)| r)
            .minmax()
            .into_option()
            .unwrap();
        let (&min_col, &max_col) = self
            .locations
            .iter()
            .map(|(_, c)| c)
            .minmax()
            .into_option()
            .unwrap();
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                write!(
                    f,
                    "{}",
                    if self.locations.contains(&(row, col)) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

const N: (isize, isize) = (-1, 0);
const NE: (isize, isize) = (-1, 1);
const E: (isize, isize) = (0, 1);
const SE: (isize, isize) = (1, 1);
const S: (isize, isize) = (1, 0);
const SW: (isize, isize) = (1, -1);
const W: (isize, isize) = (0, -1);
const NW: (isize, isize) = (-1, -1);

const COMPASS: [(isize, isize); 8] = [N, NE, E, SE, S, SW, W, NW];

impl Elves {
    fn new(input: &str) -> Self {
        let locations = HashSet::from_iter(input.lines().enumerate().flat_map(|(r, l)| {
            l.as_bytes().iter().enumerate().filter_map(move |(c, b)| {
                if *b == b'#' {
                    Some((r as isize, c as isize))
                } else {
                    None
                }
            })
        }));
        Self { step: 0, locations }
    }

    fn population_count(&self, point: (isize, isize), neighbours: &[(isize, isize)]) -> usize {
        neighbours
            .iter()
            .filter(|neighbour| {
                let check = (point.0 + neighbour.0, point.1 + neighbour.1);
                self.locations.contains(&check)
            })
            .count()
    }

    fn step(&mut self) -> bool {
        let mut proposed: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
        const CHECK_DIRECTIONS: [[(isize, isize); 3]; 4] =
            [[NE, N, NW], [SE, S, SW], [NW, W, SW], [NE, E, SE]];
        for &elf in &self.locations {
            if self.population_count(elf, &COMPASS) == 0 {
                continue;
            }

            for i in 0..CHECK_DIRECTIONS.len() {
                let check = CHECK_DIRECTIONS[(self.step + i) % CHECK_DIRECTIONS.len()];
                if self.population_count(elf, &check) != 0 {
                    continue;
                }
                let step = (check[1].0 + elf.0, check[1].1 + elf.1);

                proposed
                    .entry(step)
                    .and_modify(|v| v.push(elf))
                    .or_insert_with(|| vec![elf]);
                break;
            }
        }

        self.step += 1;

        if proposed.is_empty() {
            return false;
        }

        for (destination, proposers) in proposed {
            if proposers.len() != 1 {
                continue;
            }

            self.locations.insert(destination);
            self.locations.remove(&proposers[0]);
        }

        true
    }

    fn empty_tiles(&self) -> usize {
        let (&min_row, &max_row) = self
            .locations
            .iter()
            .map(|(r, _)| r)
            .minmax()
            .into_option()
            .unwrap();
        let (&min_col, &max_col) = self
            .locations
            .iter()
            .map(|(_, c)| c)
            .minmax()
            .into_option()
            .unwrap();
        (min_row.abs_diff(max_row) + 1) * (min_col.abs_diff(max_col) + 1) - self.locations.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut elves = Elves::new(input);
    #[cfg(feature = "demo")]
    println!("{elves}");
    for _ in 0..10 {
        elves.step();
        #[cfg(feature = "demo")]
        println!("{elves}");
    }
    Some(elves.empty_tiles())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut elves = Elves::new(input);
    for count in 1.. {
        if !elves.step() {
            return Some(count);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_simple() {
        let mut elves = Elves::new(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        println!("{elves}");
        assert_eq!(elves.empty_tiles(), 3);
        assert!(elves.step());
        println!("{elves}");
        assert_eq!(elves.empty_tiles(), 5);
        assert!(elves.step());
        println!("{elves}");
        assert_eq!(elves.empty_tiles(), 15);
        assert!(elves.step());
        println!("{elves}");
        assert_eq!(elves.empty_tiles(), 25);
        assert!(!elves.step());
        assert_eq!(elves.empty_tiles(), 25);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(110));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(20));
    }
}
