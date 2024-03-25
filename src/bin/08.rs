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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
