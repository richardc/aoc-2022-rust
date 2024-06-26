advent_of_code::solution!(5);

use std::collections::HashMap;
use std::marker::PhantomData;

use itertools::Itertools;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn new(s: &str) -> Self {
        let chunks = s.split(' ').collect_vec();
        let count = chunks[1].parse().unwrap();
        let from = chunks[3].parse().unwrap();
        let to = chunks[5].parse().unwrap();
        Self { count, from, to }
    }
}

trait Mover {}

struct CrateMover9000 {}
impl Mover for CrateMover9000 {}

struct CrateMover9001 {}
impl Mover for CrateMover9001 {}

struct Crane<M: Mover> {
    columns: Vec<Vec<char>>,
    moves: Vec<Move>,
    mover: PhantomData<M>,
}

impl<M: Mover> Crane<M> {
    fn new(s: &str) -> Self {
        let (picture, moves) = s.split_once("\n\n").unwrap();
        let moves = moves.lines().map(Move::new).collect_vec();
        let mut lines = picture.lines().rev();
        let indexes = lines.next().unwrap();
        let mut next = 0;
        let indexes: HashMap<usize, usize> =
            HashMap::from_iter(indexes.as_bytes().iter().enumerate().filter_map(|(i, b)| {
                if b.is_ascii_digit() {
                    let idx = next;
                    next += 1;
                    Some((i, idx))
                } else {
                    None
                }
            }));
        let mut columns: Vec<Vec<char>> = Vec::from_iter(indexes.iter().map(|_| Vec::new()));
        for l in lines {
            for (i, b) in l.as_bytes().iter().enumerate() {
                if b.is_ascii_uppercase() {
                    let index = indexes.get(&i).unwrap();
                    columns[*index].push(*b as char);
                }
            }
        }
        let mover = PhantomData;
        Self {
            columns,
            moves,
            mover,
        }
    }

    fn column_string(&self) -> String {
        String::from_iter(self.columns.iter().filter_map(|c| c.last()))
    }
}

impl Crane<CrateMover9000> {
    fn moves(&mut self) {
        for m in &self.moves {
            for _ in 0..m.count {
                let top = self.columns[m.from - 1].pop().unwrap();
                self.columns[m.to - 1].push(top);
            }
        }
    }
}

impl Crane<CrateMover9001> {
    fn moves(&mut self) {
        for m in &self.moves {
            let len = self.columns[m.from - 1].len();
            let top = self.columns[m.from - 1].split_off(len - m.count);
            self.columns[m.to - 1].extend(top);
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut crane = Crane::<CrateMover9000>::new(input);
    crane.moves();
    Some(crane.column_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut crane = Crane::<CrateMover9001>::new(input);
    crane.moves();
    Some(crane.column_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("MCD".to_string()));
    }
}
