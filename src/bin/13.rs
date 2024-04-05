advent_of_code::solution!(13);

use std::cmp::Ordering;

use winnow::{
    ascii::digit1,
    combinator::{alt, delimited, separated},
    prelude::*,
    token::literal,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

type Stream<'i> = &'i str;

fn packet_integer(input: &mut Stream<'_>) -> PResult<Packet> {
    let parser = digit1;

    parser
        .map(|s: &str| Packet::Integer(s.parse().unwrap()))
        .parse_next(input)
}

fn packet_list(input: &mut Stream<'_>) -> PResult<Packet> {
    let parser = delimited(
        literal("["),
        separated(0.., packet, literal(",")),
        literal("]"),
    );

    parser.map(Packet::List).parse_next(input)
}

fn packet(input: &mut Stream<'_>) -> PResult<Packet> {
    alt((packet_integer, packet_list)).parse_next(input)
}

impl Packet {
    fn new(s: &str) -> Self {
        packet.parse_peek(s).expect("parsed").1
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Integer(l), Integer(r)) => l.cmp(r),
            (List(l), List(r)) => {
                for (l, r) in l.iter().zip(r.iter()) {
                    let ord = l.cmp(r);
                    if ord != Ordering::Equal {
                        return ord;
                    }
                }
                l.len().cmp(&r.len())
            }
            (Integer(_), List(_)) => List(vec![self.clone()]).cmp(other),
            (List(_), Integer(_)) => self.cmp(&List(vec![other.clone()])),
        }
    }
}

fn correct_order(left: &Packet, right: &Packet) -> bool {
    left.cmp(right) != Ordering::Greater
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

pub fn part_two(input: &str) -> Option<usize> {
    let mut packets: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(Packet::new)
        .collect();
    let divider_one = Packet::new("[[2]]");
    let divider_two = Packet::new("[[6]]");
    packets.push(divider_one.clone());
    packets.push(divider_two.clone());
    packets.sort();
    let first = packets.iter().position(|p| *p == divider_one).unwrap();
    let second = packets.iter().position(|p| *p == divider_two).unwrap();

    Some((first + 1) * (second + 1))
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
        assert_eq!(result, Some(140));
    }
}
