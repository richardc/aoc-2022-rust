use std::collections::HashMap;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::i64,
    sequence::tuple, IResult,
};

advent_of_code::solution!(21);

type Value = i64;
type Name = [u8; 4];

enum Kind {
    Add,
    Sub,
    Div,
    Mul,
}

impl Kind {
    fn new(s: &str) -> Self {
        match s {
            "+" => Kind::Add,
            "-" => Kind::Sub,
            "*" => Kind::Mul,
            "/" => Kind::Div,
            _ => unreachable!("not an operation"),
        }
    }
}

struct Operation {
    kind: Kind,
    left: Name,
    right: Name,
}

enum Monkey {
    Value(Value),
    Op(Operation),
}

struct Monkeys {
    all: HashMap<Name, Monkey>,
}

fn monkey_value(input: &str) -> IResult<&str, Monkey> {
    let (input, value) = i64(input)?;

    Ok((input, Monkey::Value(value)))
}

fn monkey_op(input: &str) -> IResult<&str, Monkey> {
    let (input, (left, _, op, _, right)) = tuple((
        alpha1,
        tag(" "),
        alt((tag("+"), tag("-"), tag("/"), tag("*"))),
        tag(" "),
        alpha1,
    ))(input)?;

    Ok((
        input,
        Monkey::Op(Operation {
            left: (&left.as_bytes()[..4]).try_into().expect("bytes"),
            right: (&right.as_bytes()[..4]).try_into().expect("bytes"),
            kind: Kind::new(op),
        }),
    ))
}

fn monkey_line(input: &str) -> IResult<&str, (Name, Monkey)> {
    let (input, (name, _, monkey)) =
        tuple((alpha1, tag(": "), alt((monkey_value, monkey_op))))(input)?;

    Ok((
        input,
        ((&name.as_bytes()[..4]).try_into().expect("bytes"), monkey),
    ))
}

fn parse_monkey(s: &str) -> (Name, Monkey) {
    monkey_line(s).expect("parse").1
}

impl Monkeys {
    fn new(input: &str) -> Self {
        let all = HashMap::from_iter(input.lines().map(parse_monkey));
        Self { all }
    }

    fn cached_eval(&self, name: &Name, cache: &mut HashMap<Name, Value>) -> Value {
        if let Some(value) = cache.get(name) {
            return *value;
        }

        let monkey = self.all.get(name).expect("known monkey");
        let value = match &monkey {
            Monkey::Value(v) => *v,
            Monkey::Op(op) => {
                let left = self.cached_eval(&op.left, cache);
                let right = self.cached_eval(&op.right, cache);
                match op.kind {
                    Kind::Add => left + right,
                    Kind::Sub => left - right,
                    Kind::Mul => left * right,
                    Kind::Div => left / right,
                }
            }
        };

        cache.insert(*name, value);
        value
    }

    fn eval(&self, name: &Name) -> Value {
        self.cached_eval(name, &mut HashMap::new())
    }
}

pub fn part_one(input: &str) -> Option<Value> {
    let monkeys = Monkeys::new(input);
    Some(monkeys.eval(b"root"))
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
        assert_eq!(result, Some(152));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
