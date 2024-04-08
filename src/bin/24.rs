use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

use std::collections::HashSet;

advent_of_code::solution!(24);

#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(c: u8) -> Option<Self> {
        use Direction::*;
        match c {
            b'>' => Some(Right),
            b'v' => Some(Down),
            b'<' => Some(Left),
            b'^' => Some(Up),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Flake {
    point: (isize, isize),
    direction: Direction,
}

#[derive(Debug)]
struct Maze {
    rows: isize,
    columns: isize,
    flakes: HashSet<Flake>,
}

type State = ((isize, isize), isize);

impl Maze {
    fn new(input: &str) -> Self {
        let rows = input.lines().count() as isize;
        let columns = input.lines().next().unwrap().len() as isize;
        let flakes = HashSet::from_iter(input.lines().enumerate().flat_map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(column, b)| {
                    Direction::new(*b).map(|direction| Flake {
                        point: (row as isize, column as isize),
                        direction,
                    })
                })
        }));

        Self {
            rows,
            columns,
            flakes,
        }
    }

    fn intersecting_flake(&self, position: (isize, isize), time: isize) -> bool {
        // For a given time, we only need to calculate what the start position
        // of the colliding flake in that position would be.  This is some sums
        // and then 4 lookups, rather than simulating all the flakes.
        use Direction::*;
        [
            Flake {
                point: (
                    (position.0 + time - 1).rem_euclid(self.rows - 2) + 1,
                    position.1,
                ),
                direction: Up,
            },
            Flake {
                point: (
                    (position.0 - time - 1).rem_euclid(self.rows - 2) + 1,
                    position.1,
                ),
                direction: Down,
            },
            Flake {
                point: (
                    position.0,
                    (position.1 + time - 1).rem_euclid(self.columns - 2) + 1,
                ),
                direction: Left,
            },
            Flake {
                point: (
                    position.0,
                    (position.1 - time - 1).rem_euclid(self.columns - 2) + 1,
                ),
                direction: Right,
            },
        ]
        .into_iter()
        .any(|flake| self.flakes.contains(&flake))
    }

    fn out_of_bounds(&self, position: (isize, isize)) -> bool {
        if position == (0, 1) || position == (self.rows - 1, self.columns - 2) {
            return false;
        }

        if position.0 < 1
            || position.1 < 1
            || position.0 >= self.rows - 1
            || position.1 >= self.columns - 1
        {
            // on or past the walls
            return true;
        }
        // inside the walls
        false
    }

    fn legal_state(&self, state: State) -> bool {
        !self.out_of_bounds(state.0) && !self.intersecting_flake(state.0, state.1)
    }

    fn neighbours(&self, position: (isize, isize), time: isize) -> Vec<(State, usize)> {
        // For 1 minute in the future, where is it allowable to be?
        [
            position,                     // Wait
            (position.0 - 1, position.1), // Up
            (position.0 + 1, position.1), // Down
            (position.0, position.1 - 1), // Left
            (position.0, position.1 + 1), // Right
        ]
        .iter()
        .map(|&position| (position, time + 1))
        .filter(|&state| self.legal_state(state))
        .map(|state| (state, 1))
        .collect_vec()
    }

    fn pathfind(&self, start: State, goal: (isize, isize)) -> usize {
        let (_path, cost) = dijkstra(
            &start,
            |&(position, time)| self.neighbours(position, time),
            |&(position, _)| position == goal,
        )
        .expect("to find a path");
        start.1 as usize + cost
    }

    fn shortest_path(&self) -> usize {
        let start = (0, 1);
        let goal = (self.rows - 1, self.columns - 2);
        self.pathfind((start, 0), goal)
    }

    fn out_in_out(&self) -> usize {
        let start = (0, 1);
        let goal = (self.rows - 1, self.columns - 2);
        let out = self.pathfind((start, 0), goal);
        let back = self.pathfind((goal, out as isize), start);
        self.pathfind((start, back as isize), goal)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = Maze::new(input);
    Some(maze.shortest_path())
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = Maze::new(input);
    Some(maze.out_in_out())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maze_flake_collisions() {
        let maze = Maze::new(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(maze.columns, 7);
        assert_eq!(maze.rows, 7);
        assert!(maze.intersecting_flake((2, 1), 0), "right flake at 2,1");
        assert!(
            !maze.intersecting_flake((2, 1), 1),
            "right flake moved from 2,1 at t=1"
        );
        assert!(
            maze.intersecting_flake((2, 2), 1),
            "right flake moved to 2,2 at t=1"
        );
        assert!(
            maze.intersecting_flake((2, 3), 2),
            "right flake moved to 2,3 at t=2"
        );
        assert!(
            maze.intersecting_flake((2, 4), 3),
            "right flake moved to 2,4 at t=3"
        );
        assert!(
            maze.intersecting_flake((2, 5), 4),
            "right flake moved to 2,5 at t=4"
        );
        assert!(
            !maze.intersecting_flake((2, 1), 2),
            "right flake moved from 2,1 at t=2"
        );
        assert!(
            !maze.intersecting_flake((2, 1), 3),
            "right flake moved from 2,1 at t=3"
        );
        assert!(
            !maze.intersecting_flake((2, 1), 4),
            "right flake moved from 2,1 at t=4"
        );
        assert!(
            maze.intersecting_flake((2, 1), 5),
            "right flake loops at t=5"
        );

        assert!(maze.intersecting_flake((4, 4), 0), "down flake at 4,4");
        assert!(
            maze.intersecting_flake((5, 4), 1),
            "down flake moved to 5,4 at t=1"
        );
        assert!(
            !maze.intersecting_flake((4, 4), 1),
            "down flake moved from 4,4 at t=1"
        );
        assert!(
            maze.intersecting_flake((4, 4), 5),
            "down flake returned to 4,4 at t=5"
        );
    }

    #[test]
    fn maze_out_of_bounds() {
        let maze = Maze::new(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(maze.columns, 7);
        assert_eq!(maze.rows, 7);
        assert!(maze.out_of_bounds((0, 0)), "top left corner");
        assert!(!maze.out_of_bounds((0, 1)), "starting point");
        assert!(maze.out_of_bounds((0, 2)), "top wall");
        assert!(maze.out_of_bounds((1, 0)), "side wall");
        assert!(!maze.out_of_bounds((1, 1)), "corridor");
        assert!(maze.out_of_bounds((1, maze.columns)), "right wall");
        assert!(maze.out_of_bounds((maze.rows, 1)), "bottom wall");
        assert!(
            !maze.out_of_bounds((maze.rows - 1, maze.columns - 2)),
            "exit"
        );
        assert!(
            maze.out_of_bounds((maze.rows, maze.columns)),
            "bottom right corner"
        );
        assert!(maze.out_of_bounds((-1, 1)), "out of bounds up");
    }

    #[test]
    fn maze_neighbours() {
        let maze = Maze::new(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            maze.neighbours((0, 1), 0),
            vec![(((0, 1), 1), 1), (((1, 1), 1), 1)]
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
