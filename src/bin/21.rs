use std::collections::HashMap;

advent_of_code::solution!(21);

type Value = i64;

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
    Constant(Value),
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
                Monkey::Constant(value)
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
            Monkey::Constant(v) | Monkey::Variable(v) => *v,
            Monkey::Op(op) => {
                let left = op.left.eval();
                let right = op.right.eval();
                op.op.apply(left, right)
            }
        }
    }

    fn constant_fold_tree(&mut self) {
        if let Monkey::Op(node) = self {
            let Operation { left, right, op } = node.as_mut();
            left.constant_fold_tree();
            right.constant_fold_tree();
            if let (Monkey::Constant(left), Monkey::Constant(right)) = (left, right) {
                *self = Monkey::Constant(op.apply(*left, *right));
            }
        }
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

        let (constant, variable, left_was_constant) = if matches!(left, Monkey::Constant(_)) {
            (left.eval(), right, true)
        } else {
            (right.eval(), left, false)
        };

        match op {
            Op::Add => *accum -= constant,
            Op::Sub => {
                if left_was_constant {
                    *accum = constant - *accum
                } else {
                    *accum += constant
                }
            }
            Op::Mul => *accum /= constant,
            Op::Div => {
                if left_was_constant {
                    *accum = constant / *accum
                } else {
                    *accum *= constant
                }
            }
        }

        variable
    }

    fn human_say(mut self) -> Value {
        self.constant_fold_tree();

        let (left, _, right) = self.into_children();
        let (mut target, mut humn_branch) = if matches!(left, Monkey::Constant(_)) {
            (left.eval(), right)
        } else {
            (right.eval(), left)
        };

        while !matches!(humn_branch, Monkey::Variable(_)) {
            humn_branch = humn_branch.uneval(&mut target);
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
