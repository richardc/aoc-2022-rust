use std::collections::HashMap;

advent_of_code::solution!(21);

type Value = i64;
type Name = [u8; 4];

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Op {
    fn new(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => unreachable!("not an operation"),
        }
    }

    fn apply(&self, left: Value, right: Value) -> Value {
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        }
    }
}

#[derive(Debug)]
struct Operation {
    op: Op,
    left: Monkey,
    right: Monkey,
}

#[derive(Debug)]
enum Monkey {
    Variable(Value),
    Value(Value),
    Op(Box<Operation>),
}

fn parse_monkeys(input: &str) -> Monkey {
    let named: HashMap<&str, &str> =
        HashMap::from_iter(input.lines().map(|l| l.split_once(": ").unwrap()));

    Monkey::from_str("root", &named)
}

impl Monkey {
    fn from_str(name: &str, dict: &HashMap<&str, &str>) -> Self {
        let def = dict.get(name).unwrap();
        if let Ok(value) = def.parse::<Value>() {
            if name == "humn" {
                Monkey::Variable(value)
            } else {
                Monkey::Value(value)
            }
        } else {
            let args: Vec<_> = def.split(' ').collect();
            let left = Monkey::from_str(args[0], dict);
            let op = Op::new(args[1]);
            let right = Monkey::from_str(args[2], dict);
            Monkey::Op(Box::new(Operation { left, right, op }))
        }
    }

    fn eval(&self) -> Value {
        match &self {
            Monkey::Value(v) | Monkey::Variable(v) => *v,
            Monkey::Op(op) => {
                let left = op.left.eval();
                let right = op.right.eval();
                op.op.apply(left, right)
            }
        }
    }

    fn fold_tree(&mut self) {
        if let Monkey::Op(node) = self {
            let Operation { left, right, op } = node.as_mut();
            left.fold_tree();
            right.fold_tree();
            if let (Monkey::Value(left), Monkey::Value(right)) = (left, right) {
                *self = Monkey::Value(op.apply(*left, *right));
            }
        }
    }

    fn have_humn(&self) -> bool {
        if let Monkey::Variable(_) = self {
            return true;
        }
        if let Monkey::Op(node) = self {
            if node.left.have_humn() {
                return true;
            }
            if node.right.have_humn() {
                return true;
            }
        }
        false
    }

    fn into_children(self) -> (Monkey, Op, Monkey) {
        if let Monkey::Op(node) = self {
            let Operation { left, op, right } = *node;
            (left, op, right)
        } else {
            unreachable!("should only be called on an op")
        }
    }

    fn uneval(self, accum: &mut Value) -> Self {
        let (left, op, right) = self.into_children();

        left
    }

    fn human_say(mut self) -> Value {
        self.fold_tree();
        let Monkey::Op(node) = self else {
            unreachable!("should only be called on an op");
        };

        let Operation { left, right, .. } = *node;
        let (constant, mut humn) = if left.have_humn() {
            (right, left)
        } else {
            (left, right)
        };

        let mut target = constant.eval();
        while !matches!(humn, Monkey::Variable(_)) {
            humn = humn.uneval(&mut target);
        }
        target
    }
}

pub fn part_one(input: &str) -> Option<Value> {
    let root = parse_monkeys(input);
    Some(root.eval())
}

pub fn part_two(input: &str) -> Option<Value> {
    let root = parse_monkeys(input);
    Some(root.human_say())
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
        assert_eq!(result, Some(301));
    }
}
