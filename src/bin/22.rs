use std::collections::HashMap;

use winnow::ascii::dec_uint;
use winnow::combinator::{alt, repeat};
use winnow::prelude::*;

advent_of_code::solution!(22);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

trait Zooper {
    fn step(&self, point: (usize, usize), direction: Facing) -> ((usize, usize), Facing);
}

struct Walker {
    row: usize,
    column: usize,
    facing: Facing,
    zooper: Box<dyn Zooper>,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
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
    fn new(map: &Map, zooper: Box<dyn Zooper>) -> Self {
        Self {
            row: map.start.0,
            column: map.start.1,
            facing: Facing::Right,
            zooper,
        }
    }

    fn walk(&mut self, map: &Map) {
        for step in parse_steps
            .parse(&map.directions)
            .expect("parsed directions")
        {
            use Step::*;
            match step {
                TurnRight => self.facing.turn_right(),
                TurnLeft => self.facing.turn_left(),
                Forward(steps) => {
                    for _ in 0..steps {
                        self.step(map);
                    }
                }
            }
        }
    }

    fn step(&mut self, map: &Map) {
        let next = self.facing.step((self.row, self.column));
        let (next, facing) = if map.tiles.contains_key(&next) {
            (next, self.facing)
        } else {
            self.zooper.step((self.row, self.column), self.facing)
        };
        let tile = map.tiles.get(&next).unwrap();

        match tile {
            Tile::Open => {
                (self.row, self.column) = next;
                self.facing = facing;
            }
            Tile::Rock => (),
        }
    }

    fn sum(self) -> usize {
        self.row * 1000 + self.column * 4 + self.facing as usize
    }
}

struct Pacman {
    map: Map,
}

impl Pacman {
    fn new(map: &Map) -> Self {
        Self { map: map.clone() }
    }
}

impl Zooper for Pacman {
    fn step(&self, point: (usize, usize), direction: Facing) -> ((usize, usize), Facing) {
        // Pacman rules - step back until we hit void again
        let mut last = point;
        let mut next = last;
        while self.map.tiles.contains_key(&next) {
            last = next;
            next = direction.step_backwards(next);
        }
        (last, direction)
    }
}

#[derive(Debug, Default)]
struct Face {
    position: (usize, usize),
    connections: HashMap<Facing, (usize, Facing)>,
}

struct CubeNet {
    width: usize,
    faces: [Face; 6],
}

// There are 11 Nets of a cube, we only have encoded 2.   A future enhancement might be to
// generate the other 9 and search for a match.  That or we could write a routine that would
// generate a net from a raw map, as that wouldn't need to account for the rotations.
impl CubeNet {
    fn new(map: &Map) -> Self {
        if map.tiles.len() == 4 * 4 * 6 {
            Self::example_net()
        } else {
            Self::puzzle_net()
        }
    }

    fn puzzle_net() -> Self {
        use Facing::*;
        // The net from my puzzle input
        //  | 0 1 2
        //--+------
        // 0|   0 1
        // 1|   2
        // 2| 3 4
        // 3| 5
        Self {
            width: 50,
            faces: [
                // 0
                Face {
                    position: (0, 1),
                    connections: HashMap::from_iter([(Up, (5, Right)), (Left, (3, Right))]),
                },
                // 1
                Face {
                    position: (0, 2),
                    connections: HashMap::from_iter([
                        (Down, (2, Left)),
                        (Up, (5, Up)),
                        (Right, (4, Left)),
                    ]),
                },
                // 2
                Face {
                    position: (1, 1),
                    connections: HashMap::from_iter([(Left, (3, Down)), (Right, (1, Up))]),
                },
                // 3
                Face {
                    position: (2, 0),
                    connections: HashMap::from_iter([(Up, (2, Right)), (Left, (0, Right))]),
                },
                // 4
                Face {
                    position: (2, 1),
                    connections: HashMap::from_iter([(Right, (1, Left)), (Down, (5, Left))]),
                },
                // 5
                Face {
                    position: (3, 0),
                    connections: HashMap::from_iter([
                        (Left, (0, Down)),
                        (Right, (4, Up)),
                        (Down, (1, Down)),
                    ]),
                },
            ],
        }
    }

    fn example_net() -> Self {
        use Facing::*;
        // The net of the example
        //    0 1 2 3
        // 0      0
        // 1  1 2 3
        // 2      4 5
        Self {
            width: 4,
            faces: [
                // 0
                Face {
                    position: (0, 2),
                    connections: HashMap::from_iter([
                        (Left, (2, Down)),
                        (Right, (5, Left)),
                        (Up, (1, Down)),
                    ]),
                },
                // 1
                Face {
                    position: (1, 0),
                    connections: HashMap::from_iter([
                        (Left, (5, Up)),
                        (Up, (0, Down)),
                        (Down, (4, Up)),
                    ]),
                },
                // 2
                Face {
                    position: (1, 1),
                    connections: HashMap::from_iter([(Up, (0, Right)), (Down, (4, Right))]),
                },
                // 3
                Face {
                    position: (1, 2),
                    connections: HashMap::from_iter([(Right, (5, Down))]),
                },
                // 4
                Face {
                    position: (2, 2),
                    connections: HashMap::from_iter([(Left, (2, Up)), (Down, (1, Up))]),
                },
                // 5
                Face {
                    position: (2, 3),
                    connections: HashMap::from_iter([
                        (Right, (0, Left)),
                        (Up, (3, Left)),
                        (Down, (1, Right)),
                    ]),
                },
            ],
        }
    }
}

impl Zooper for CubeNet {
    fn step(&self, point: (usize, usize), direction: Facing) -> ((usize, usize), Facing) {
        // Find the face we're starting on
        let face_position = ((point.0 - 1) / self.width, (point.1 - 1) / self.width);

        let face = self
            .faces
            .iter()
            .find(|face| face.position == face_position)
            .expect("should be a face in the net");
        let (next_face, next_direction) = face
            .connections
            .get(&direction)
            .expect("should have a connection on an edge");

        let next_face = &self.faces[*next_face];

        // Map from 1-based global coordinates to 0-based face coordinates
        let (face_row, face_column) = ((point.0 - 1) % self.width, (point.1 - 1) % self.width);
        let last = self.width - 1;

        use Facing::*;
        let (next_face_row, next_face_column) = match (direction, next_direction) {
            (Right, Right) => (face_row, 0),
            (Left, Left) => (face_row, last),
            (Up, Up) => (last, face_column),
            (Down, Down) => (0, face_column),
            (Right, Left) => (last - face_row, last),
            (Left, Right) => (last - face_row, 0),
            (Up, Down) => (0, last - face_column),
            (Down, Up) => (last, last - face_column),
            (Right, Down) => (0, last - face_row),
            (Down, Left) => (face_column, last),
            (Left, Up) => (last, last - face_row),
            (Up, Right) => (face_column, 0),
            (Right, Up) => (last, face_row),
            (Down, Right) => (last - face_column, 0),
            (Left, Down) => (0, face_row),
            (Up, Left) => (last - face_column, last),
        };

        // Map back to the global 1-indexed position
        let next_point = (
            next_face_row + next_face.position.0 * self.width + 1,
            next_face_column + next_face.position.1 * self.width + 1,
        );
        (next_point, *next_direction)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = Map::new(input);
    let mut walker = Walker::new(&map, Box::new(Pacman::new(&map)));
    walker.walk(&map);
    Some(walker.sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = Map::new(input);
    let mut walker = Walker::new(&map, Box::new(CubeNet::new(&map)));
    walker.walk(&map);
    Some(walker.sum())
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
        assert_eq!(result, Some(5031));
    }

    mod cubenet {
        use super::*;
        use rstest::rstest;
        use Facing::*;

        #[rstest]
        #[case::example_a_right_b(((6,12), Right), ((9,15), Down))]
        #[case::example_b_up_a(((9,15), Up), ((6,12), Left))]
        #[case::example_c_down_d(((12,11), Down), ((8,2), Up))]
        #[case::example_d_up_c(((8,2), Down), ((12,11), Up))]
        #[case::north_wrap(((1,9), Up), ((5,4), Down))]
        #[case::north_wrap_inverse(((5,4), Up), ((1,9), Down))]
        #[case::right_left(((2,12), Right), ((11,16), Left))]
        #[case::right_left_inverse(((11,16), Right), ((2,12), Left))]
        #[case::left_down(((1,9), Left), ((5,5), Down))]
        #[case::left_down_inverse(((5,5), Up), ((1,9), Right))]
        #[case::down_right(((12,16), Down), ((5,1), Right))]
        #[case::down_right_inverse(((5,1), Left), ((12,16), Up))]
        fn test_example_zoop(
            #[case] start: ((usize, usize), Facing),
            #[case] expected: ((usize, usize), Facing),
        ) {
            let zooper = CubeNet::example_net();
            let result = zooper.step(start.0, start.1);
            assert_eq!(result, expected);
        }

        #[rstest]
        #[case::down_left(((50,150), Down), ((100,100), Left))]
        #[case::down_left_inverse(((100,100), Right), ((50,150), Up))]
        #[case::left_right(((1,51), Left), ((150,1), Right))]
        #[case::left_right_inverse(((150,1), Left), ((1,51), Right))]
        fn test_puzzle_zoop(
            #[case] start: ((usize, usize), Facing),
            #[case] expected: ((usize, usize), Facing),
        ) {
            let zooper = CubeNet::puzzle_net();
            let result = zooper.step(start.0, start.1);
            assert_eq!(result, expected);
        }
    }
}
