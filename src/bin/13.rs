advent_of_code::solution!(13);

use std::cmp::Ordering;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

#[derive(Debug, Clone)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

fn packet_integer(input: &str) -> IResult<&str, Packet> {
    let parser = digit1;

    map(parser, |s: &str| Packet::Integer(s.parse().unwrap()))(input)
}

fn packet_list(input: &str) -> IResult<&str, Packet> {
    let parser = delimited(tag("["), separated_list0(tag(","), packet), tag("]"));

    map(parser, Packet::List)(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((packet_integer, packet_list))(input)
}

impl Packet {
    fn new(s: &str) -> Self {
        packet(s).expect("parsed").1
    }
}

fn correct_order(left: &Packet, right: &Packet) -> bool {
    // Coorce None (we couldn't find a difference), to true (no difference)
    if let Some(false) = correct_order_inner(left, right) {
        false
    } else {
        true
    }
}

fn correct_order_inner(left: &Packet, right: &Packet) -> Option<bool> {
    use Packet::*;
    match (left, right) {
        (Integer(l), Integer(r)) => match l.cmp(r) {
            Ordering::Less => Some(true),
            Ordering::Greater => Some(false),
            Ordering::Equal => None,
        },
        (List(l), List(r)) => {
            for i in 0..(l.len().min(r.len())) {
                if let Some(decision) = correct_order_inner(&l[i], &r[i]) {
                    return Some(decision);
                }
            }
            match l.len().cmp(&r.len()) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            }
        }
        (Integer(_), List(_)) => correct_order_inner(&List(vec![left.clone()]), right),

        (List(_), Integer(_)) => correct_order_inner(left, &List(vec![right.clone()])),
    }
}

#[derive(Debug)]
struct Packets {
    packets: Vec<(Packet, Packet)>,
}

impl Packets {
    fn new(s: &str) -> Self {
        let packets = s
            .split("\n\n")
            .map(|pair| {
                let (left, right) = pair.split_once('\n').expect("pair is two lines");
                let left = Packet::new(left);
                let right = Packet::new(right);
                (left, right)
            })
            .collect();
        Self { packets }
    }

    fn correct_order(&self) -> usize {
        self.packets
            .iter()
            .enumerate()
            .filter(|(_, pair)| correct_order(&pair.0, &pair.1))
            .map(|(i, _)| i + 1)
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let packets = Packets::new(input);
    Some(packets.correct_order())
}

pub fn part_two(input: &str) -> Option<u32> {
    _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_order_one() {
        let left = Packet::new("[1,1,3,1,1]");
        let right = Packet::new("[1,1,5,1,1]");
        assert!(correct_order(&left, &right));
    }

    #[test]
    fn test_correct_order_two() {
        let left = Packet::new("[[1],[2,3,4]]");
        let right = Packet::new("[[1],4]");
        assert!(correct_order(&left, &right));
    }

    #[test]
    fn test_correct_order_three() {
        let left = Packet::new("[9]");
        let right = Packet::new("[[8,7,6]]");
        assert!(!correct_order(&left, &right));
    }

    #[test]
    fn test_correct_order_four() {
        let left = Packet::new("[[4,4],4,4]");
        let right = Packet::new("[[4,4],4,4,4]]");
        assert!(correct_order(&left, &right));
    }

    #[test]
    fn test_correct_order_five() {
        let left = Packet::new("[7,7,7,7]");
        let right = Packet::new("[7,7,7]");
        assert!(!correct_order(&left, &right));
    }

    #[test]
    fn test_correct_order_six() {
        let left = Packet::new("[]");
        let right = Packet::new("[3]");
        assert!(correct_order(&left, &right));
    }

    #[test]
    fn test_correct_order_seven() {
        let left = Packet::new("[[[]]]");
        let right = Packet::new("[[]]");
        assert!(!correct_order(&left, &right));
    }

    #[test]
    fn test_correct_order_eight() {
        let left = Packet::new("[1,[2,[3,[4,[5,6,7]]]],8,9]");
        let right = Packet::new("[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert!(!correct_order(&left, &right));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
