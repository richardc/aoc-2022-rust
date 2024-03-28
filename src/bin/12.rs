advent_of_code::solution!(12);

use pathfinding::prelude::*;

#[derive(Debug)]
struct Maze {
    cells: Matrix<char>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn new(s: &str) -> Self {
        let mut cells = Matrix::from_rows(s.lines().map(|l| l.chars())).expect("parsed a Matric");
        let mut start = (0, 0);
        let mut end = (0, 0);
        for ((r, c), ch) in cells.items_mut() {
            if *ch == 'E' {
                end = (r, c);
                *ch = 'z';
            }
            if *ch == 'S' {
                start = (r, c);
                *ch = 'a';
            }
        }

        Self { cells, start, end }
    }

    fn neighbours(&self, pos: (usize, usize)) -> Vec<((usize, usize), usize)> {
        let height = *self.cells.get(pos).expect("to have a height") as i8;
        self.cells
            .neighbours(pos, false)
            .filter(|&neighbour| {
                let n_height = *self.cells.get(neighbour).expect("neighbour exists") as i8;
                let diff = n_height - height;
                diff <= 1
            })
            .map(|neighbour| (neighbour, 1))
            .collect()
    }

    fn shortest_path(&self) -> usize {
        let (_path, cost) = dijkstra(&self.start, |&p| self.neighbours(p), |&p| p == self.end)
            .expect("there will be a path");
        cost
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = Maze::new(input);
    Some(maze.shortest_path())
}

pub fn part_two(input: &str) -> Option<usize> {
    _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
