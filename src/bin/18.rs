advent_of_code::solution!(18);

use itertools::{iproduct, Itertools};

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

impl Cube {
    fn new(s: &str) -> Self {
        let (x, y, z) = sscanf::sscanf!(s, "{usize},{usize},{usize}").expect("cube");
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

use ndarray::Array3;

#[derive(Debug, Default, PartialEq, Clone)]
enum Block {
    #[default]
    Air,
    Lava,
    Void,
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut space: Array3<Block> = Array3::default((25, 25, 25));
    for Cube { x, y, z } in input.lines().map(Cube::new) {
        space[[x + 1, y + 1, z + 1]] = Block::Lava;
    }

    // Turn all the inner air into nothing, apart from the outer border.
    for (x, y, z) in iproduct!(1..23, 1..23, 1..23) {
        if space[[x, y, z]] == Block::Air {
            space[[x, y, z]] = Block::Void;
        }
    }

    let mut next = space.clone();

    // Flood fill, but in the style of Life.  Just flip any Void touching Air to Air
    loop {
        for (x, y, z) in iproduct!(1..23, 1..23, 1..23) {
            if space[[x, y, z]] == Block::Void {
                if space[[x - 1, y, z]] == Block::Air
                    || space[[x + 1, y, z]] == Block::Air
                    || space[[x, y - 1, z]] == Block::Air
                    || space[[x, y + 1, z]] == Block::Air
                    || space[[x, y, z - 1]] == Block::Air
                    || space[[x, y, z + 1]] == Block::Air
                {
                    next[[x, y, z]] = Block::Air
                }
            }
        }

        // We'll be done when there's no more Void to flip
        if next == space {
            break;
        }
        space = next.clone();
    }

    // Now for every block, count up the faces touching Air
    let mut count = 0;
    for (x, y, z) in iproduct!(1..23, 1..23, 1..23) {
        if next[[x, y, z]] == Block::Lava {
            if next[[x - 1, y, z]] == Block::Air {
                count += 1;
            }
            if next[[x + 1, y, z]] == Block::Air {
                count += 1;
            }
            if next[[x, y - 1, z]] == Block::Air {
                count += 1;
            }
            if next[[x, y + 1, z]] == Block::Air {
                count += 1;
            }
            if next[[x, y, z - 1]] == Block::Air {
                count += 1;
            }
            if next[[x, y, z + 1]] == Block::Air {
                count += 1;
            }
        }
    }

    Some(count)
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
        assert_eq!(result, Some(58));
    }
}
