use std::collections::HashMap;

use winnow::ascii::dec_uint;
use winnow::combinator::{alt, repeat};
use winnow::prelude::*;

advent_of_code::solution!(22);

#[derive(Debug, Default)]
enum Facing {
    #[default]
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    fn turn_left(&mut self) {
        use Facing::*;
        *self = match self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right,
        }
    }

    fn turn_right(&mut self) {
        use Facing::*;
        *self = match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }

    fn vector(&self) -> (isize, isize) {
        use Facing::*;
        match self {
            Right => (0, 1),
            Left => (0, -1),
            Down => (1, 0),
            Up => (-1, 0),
        }
    }

    fn step(&self, (row, column): (usize, usize)) -> (usize, usize) {
        let (vr, vc) = self.vector();
        (
            row.saturating_add_signed(vr),
            column.saturating_add_signed(vc),
        )
    }

    fn step_backwards(&self, (row, column): (usize, usize)) -> (usize, usize) {
        let (vr, vc) = self.vector();
        (
            row.saturating_add_signed(-vr),
            column.saturating_add_signed(-vc),
        )
    }
}

#[derive(Debug)]
enum Step {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

fn parse_steps(input: &mut &str) -> PResult<Vec<Step>> {
    repeat(
        1..,
        alt((
            'L'.map(|_| Step::TurnLeft),
            'R'.map(|_| Step::TurnRight),
            dec_uint.map(Step::Forward),
        )),
    )
    .parse_next(input)
}

#[derive(Debug)]
struct Walker {
    row: usize,
    column: usize,
    facing: Facing,
}

#[derive(Debug)]
enum Tile {
    Open,
    Rock,
}

impl Tile {
    fn new(c: u8) -> Option<Self> {
        use Tile::*;
        match c {
            b'.' => Some(Open),
            b'#' => Some(Rock),
            _ => None,
        }
    }
}

struct Map {
    tiles: HashMap<(usize, usize), Tile>,
    directions: String,
    start: (usize, usize),
}

impl Map {
    fn new(input: &str) -> Self {
        let mut start: Option<(usize, usize)> = None;
        let (map, directions) = input.split_once("\n\n").unwrap();
        let directions = directions.trim().to_owned();
        let mut tiles = HashMap::new();
        for (row, l) in map.lines().enumerate() {
            for (col, c) in l.as_bytes().iter().enumerate() {
                if let Some(tile) = Tile::new(*c) {
                    //eprintln!("{row} {col} {tile:?}");
                    tiles.insert((row + 1, col + 1), tile);
                    if start.is_none() {
                        start = Some((row + 1, col + 1));
                    }
                }
            }
        }
        Self {
            start: start.unwrap(),
            tiles,
            directions,
        }
    }
}

impl Walker {
    fn walk(map: &Map) -> Self {
        let mut walker = Self {
            row: map.start.0,
            column: map.start.1,
            facing: Facing::Right,
        };

        for step in parse_steps
            .parse(&map.directions)
            .expect("parsed directions")
        {
            use Step::*;
            match step {
                TurnRight => walker.facing.turn_right(),
                TurnLeft => walker.facing.turn_left(),
                Forward(steps) => {
                    for _ in 0..steps {
                        walker.step(map);
                    }
                }
            }
        }

        walker
    }

    fn step(&mut self, map: &Map) {
        let next = self.facing.step((self.row, self.column));
        let (tile, next) = if let Some(tile) = map.tiles.get(&next) {
            (tile, next)
        } else {
            // Pacman rules - step back until we hit void again
            let mut last = (self.row, self.column);
            let mut next = last;
            let mut tile = map.tiles.get(&last).expect("should start on a tile");
            while let Some(backwards) = map.tiles.get(&next) {
                last = next;
                next = self.facing.step_backwards(next);
                tile = backwards;
            }
            (tile, last)
        };
        match tile {
            Tile::Open => (self.row, self.column) = next,
            Tile::Rock => (),
        }
    }

    fn sum(self) -> usize {
        self.row * 1000 + self.column * 4 + self.facing as usize
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::new(input);
    let walked = Walker::walk(&map);
    Some(walked.sum())
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
        assert_eq!(result, Some(6032));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
