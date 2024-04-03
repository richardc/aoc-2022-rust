advent_of_code::solution!(18);

use itertools::iproduct;
use ndarray::Array3;

#[derive(Debug, Default, PartialEq, Clone)]
enum Block {
    #[default]
    Air,
    Lava,
    Void,
}

fn load(input: &str) -> Array3<Block> {
    let mut space: Array3<Block> = Array3::default((25, 25, 25));
    for line in input.lines() {
        let (x, y, z) = sscanf::sscanf!(line, "{usize},{usize},{usize}").expect("cube");
        space[[x + 1, y + 1, z + 1]] = Block::Lava;
    }
    space
}

fn external_faces(space: &Array3<Block>) -> usize {
    // For every Lava block, count up the faces touching Air
    let mut count = 0;
    for (x, y, z) in iproduct!(1..23, 1..23, 1..23) {
        if space[[x, y, z]] == Block::Lava {
            if space[[x - 1, y, z]] == Block::Air {
                count += 1;
            }
            if space[[x + 1, y, z]] == Block::Air {
                count += 1;
            }
            if space[[x, y - 1, z]] == Block::Air {
                count += 1;
            }
            if space[[x, y + 1, z]] == Block::Air {
                count += 1;
            }
            if space[[x, y, z - 1]] == Block::Air {
                count += 1;
            }
            if space[[x, y, z + 1]] == Block::Air {
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let space = load(input);
    Some(external_faces(&space))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut space = load(input);

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
            if space[[x, y, z]] == Block::Void
                && (space[[x - 1, y, z]] == Block::Air
                    || space[[x + 1, y, z]] == Block::Air
                    || space[[x, y - 1, z]] == Block::Air
                    || space[[x, y + 1, z]] == Block::Air
                    || space[[x, y, z - 1]] == Block::Air
                    || space[[x, y, z + 1]] == Block::Air)
            {
                next[[x, y, z]] = Block::Air
            }
        }

        // We'll be done when there's no more Void to flip
        if next == space {
            break;
        }
        space = next.clone();
    }

    Some(external_faces(&next))
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
