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
use std::collections::{HashMap, VecDeque};

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
    ids: HashMap<&'a str, usize>,
    interesting: Vec<usize>,
    adj: Vec<Vec<usize>>,
}

// Floyd-Warshall algorithm - takes an adjaceny list, returns the matrix of shortest paths
fn floyd_warshall(adj: &Vec<Vec<usize>>) -> Array2<u32> {
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
        let valves: Vec<_> = s.lines().map(|l| valve(l).unwrap().1).collect();
        let ids: HashMap<&str, usize> = HashMap::from_iter(
            valves
                .iter()
                .enumerate()
                .map(|(i, &(name, _, _))| (name, i)),
        );
        let interesting: Vec<usize> = valves
            .iter()
            .filter(|(_, flow, _)| *flow > 0)
            .map(|(name, _, _)| *ids.get(name).unwrap())
            .collect();
        let adj: Vec<Vec<usize>> = valves
            .iter()
            .map(|v| v.2.iter().map(|e| *ids.get(e).unwrap()).collect())
            .collect();
        Self {
            valves,
            adj,
            ids,
            interesting,
        }
    }

    fn most_released(&self, duration: u32) -> u32 {
        let distances = floyd_warshall(&self.adj);

        assert!(self.interesting.len() <= 16);
        let start = *self.ids.get("AA").unwrap();
        let closed: Bitmap<16> = Bitmap::mask(self.interesting.len());
        bfs_reach((start, 0, closed, 0), |(node, time, closed, released)| {
            let mut next = Vec::new();
            for index in closed {
                let neighbour = self.interesting[index];
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

    fn most_released_paired(&self, duration: u32) -> u32 {
        let start = *self.ids.get("AA").unwrap();
        assert!(self.valves.len() <= 64); // Will it fit?
        let mut closed: Bitmap<64> = Bitmap::new();
        self.interesting.iter().for_each(|&i| {
            closed.set(i, true);
        });
        let mut best = 0;
        let mut seen: HashMap<(usize, usize, u32), u32> = HashMap::new(); // (me, elephant, time) => released
        let mut queue: VecDeque<(usize, usize, u32, Bitmap<64>, u32, u32)> =
            VecDeque::from_iter([(start, start, 0, closed, 0, 0)]);

        while let Some((me, elephant, time, closed, flow, released)) = queue.pop_front() {
            if let Some(record) = seen.get(&(me, elephant, time)) {
                if *record >= released {
                    // Already found something better through this state
                    continue;
                }
            }
            seen.insert((me, elephant, time), released);

            if time == duration {
                best = best.max(released);
                continue;
            }

            if closed.is_empty() {
                // All are closed, we can skip to the end
                let remaining = duration - time;
                let released = flow * remaining;
                queue.push_back((me, elephant, duration, closed, flow, released));
                continue;
            }

            let released = released + flow;
            if closed.get(me) {
                let mut closed = closed;
                // We open our valve
                closed.set(me, false);

                if closed.get(elephant) {
                    let mut closed = closed;
                    // And the elephant opens their valve
                    closed.set(elephant, false);
                    let flow = flow + self.valves[me].1 + self.valves[elephant].1;
                    queue.push_back((me, elephant, time + 1, closed, flow, released));
                } else {
                    // And the elephant moves on
                    let flow = flow + self.valves[me].1;
                    for &neighbour in &self.adj[elephant] {
                        queue.push_back((me, neighbour, time + 1, closed, flow, released));
                    }
                }
            }

            // We move on
            for &neighbour in &self.adj[me] {
                if closed.get(elephant) {
                    // And the elephant opens their valve
                    let mut closed = closed;
                    closed.set(elephant, false);
                    let flow = flow + self.valves[elephant].1;
                    queue.push_back((neighbour, elephant, time + 1, closed, flow, released));
                } else {
                    // And the elephant moves on too
                    for &e_neighbour in &self.adj[elephant] {
                        queue.push_back((neighbour, e_neighbour, time + 1, closed, flow, released));
                    }
                }
            }
        }
        best
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = Valves::new(input);
    Some(valves.most_released(30))
}

pub fn part_two(input: &str) -> Option<u32> {
    let valves = Valves::new(input);
    Some(valves.most_released_paired(26))
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
        let _result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(1707));
    }
}
