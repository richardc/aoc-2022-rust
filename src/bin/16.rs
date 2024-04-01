use bitmaps::Bitmap;
use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u32},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use pathfinding::directed::bfs::bfs_reach;
use std::collections::HashMap;

advent_of_code::solution!(16);

fn valve(input: &str) -> IResult<&str, (&str, u32, Vec<&str>)> {
    let (input, (_, id, _, flow, _, edges)) = tuple((
        tag("Valve "),
        alpha1,
        tag(" has flow rate="),
        u32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list0(tag(", "), alpha1),
    ))(input)?;

    Ok((input, (id, flow, edges)))
}

struct Valves<'a> {
    valves: Vec<(&'a str, u32, Vec<&'a str>)>,
}

// Floyd-Warshall algorithm - takes an adjaceny list, returns the matrix of shortest paths
fn floyd_warshall(adj: Vec<Vec<usize>>) -> Array2<u32> {
    let mut distances = Array2::from_elem((adj.len(), adj.len()), u32::MAX);
    for i in 0..adj.len() {
        distances[[i, i]] = 0;
    }
    for i in 0..adj.len() {
        for &j in &adj[i] {
            distances[[i, j]] = 1;
            distances[[j, i]] = 1;
        }
    }
    for k in 0..adj.len() {
        for i in 0..adj.len() {
            for j in 0..adj.len() {
                if let Some(distance) = distances[[i, k]].checked_add(distances[[k, j]]) {
                    if distances[[i, j]] > distance {
                        distances[[i, j]] = distance;
                    }
                }
            }
        }
    }
    distances
}

impl<'b> Valves<'b> {
    fn new<'a>(s: &'a str) -> Valves<'a>
    where
        'a: 'b,
        'b: 'a,
    {
        let valves = s.lines().map(|l| valve(l).unwrap().1).collect();
        Self { valves }
    }

    fn most_released(&self, duration: u32) -> u32 {
        let ids: HashMap<&str, usize> = HashMap::from_iter(
            self.valves
                .iter()
                .enumerate()
                .map(|(i, &(name, _, _))| (name, i)),
        );
        let adj: Vec<Vec<usize>> = self
            .valves
            .iter()
            .map(|v| v.2.iter().map(|e| *ids.get(e).unwrap()).collect())
            .collect();
        let distances = floyd_warshall(adj);

        let interesting: Vec<usize> = self
            .valves
            .iter()
            .filter(|(_, flow, _)| *flow > 0)
            .map(|(name, _, _)| *ids.get(name).unwrap())
            .collect();

        assert!(interesting.len() <= 16);
        let start = *ids.get("AA").unwrap();
        let closed: Bitmap<16> = Bitmap::mask(interesting.len());
        bfs_reach((start, 0, closed, 0), |(node, time, closed, released)| {
            let mut next = Vec::new();
            for index in closed {
                let neighbour = interesting[index];
                let time = time + distances[[*node, neighbour]] + 1;
                if time > duration {
                    continue;
                }
                let mut closed = *closed;
                closed.set(index, false);
                let flow_added = self.valves[neighbour].1;
                let remaining_time = duration - time;
                let released = released + remaining_time * flow_added;
                next.push((neighbour, time, closed, released))
            }
            next
        })
        .map(|(_, _, _, released)| released)
        .max()
        .expect("to have a max")
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = Valves::new(input);
    Some(valves.most_released(30))
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
        assert_eq!(result, Some(1651));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
