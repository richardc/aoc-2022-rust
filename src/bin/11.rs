use itertools::Itertools;
use std::collections::VecDeque;

advent_of_code::solution!(11);

type Value = u32;

#[derive(Debug, Default)]
enum Operation {
    #[default]
    Square,
    Add(Value),
    Mul(Value),
}

impl Operation {
    fn apply(&self, old: Value) -> Value {
        use Operation::*;
        match self {
            Add(value) => old + value,
            Mul(value) => old * value,
            Square => old * old,
        }
    }
}

#[derive(Debug, Default)]
struct Monkey {
    items: VecDeque<Value>,
    operation: Operation,
    test: Value,
    matches: usize,
    otherwise: usize,
    inspected: usize,
}

impl Monkey {
    fn new(s: &str) -> Self {
        let mut monkey = Self::default();

        for line in s.lines() {
            let line = line.trim_start();
            if let Some(items) = line.strip_prefix("Starting items: ") {
                monkey
                    .items
                    .extend(items.split(", ").map(|i| i.parse::<Value>().unwrap()));
            }

            if let Some(operation) = line.strip_prefix("Operation: new = old") {
                if let Some(constant) = operation.strip_prefix(" + ") {
                    monkey.operation = Operation::Add(constant.parse().unwrap());
                }
                if let Some(constant) = operation.strip_prefix(" * ") {
                    if constant == "old" {
                        monkey.operation = Operation::Square;
                    } else {
                        monkey.operation = Operation::Mul(constant.parse().unwrap());
                    }
                }
            }

            if let Some(test) = line.strip_prefix("Test: divisible by ") {
                monkey.test = test.parse().unwrap();
            }

            if let Some(id) = line.strip_prefix("If true: throw to monkey ") {
                monkey.matches = id.parse().unwrap();
            }

            if let Some(id) = line.strip_prefix("If false: throw to monkey ") {
                monkey.otherwise = id.parse().unwrap();
            }
        }
        monkey
    }
}

#[derive(Debug)]
struct Puzzle {
    monkeys: Vec<Monkey>,
}

impl Puzzle {
    fn new(s: &str) -> Self {
        let monkeys = s.split("\n\n").map(Monkey::new).collect();
        Self { monkeys }
    }

    fn step(&mut self) {
        for i in 0..self.monkeys.len() {
            let monkey = &mut self.monkeys[i];
            let mut throw = VecDeque::new();
            while let Some(value) = monkey.items.pop_front() {
                monkey.inspected += 1;
                let value = monkey.operation.apply(value);
                let value = value / 3;
                let target = if value % monkey.test == 0 {
                    monkey.matches
                } else {
                    monkey.otherwise
                };

                throw.push_back((target, value));
            }

            while let Some((target, value)) = throw.pop_front() {
                self.monkeys[target].items.push_back(value);
            }
        }
    }

    fn monkey_business(&self) -> usize {
        self.monkeys
            .iter()
            .map(|m| m.inspected)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut puzzle = Puzzle::new(input);
    for _ in 0..20 {
        puzzle.step();
    }
    Some(puzzle.monkey_business())
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
        assert_eq!(result, Some(10605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
