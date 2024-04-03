use std::collections::BinaryHeap;
use std::collections::HashMap;

advent_of_code::solution!(19);

#[derive(Debug)]
struct Blueprint {
    number: usize,
    ore_robot_ore: usize,
    clay_robot_ore: usize,
    obsidian_robot_ore: usize,
    obsidian_robot_clay: usize,
    geode_robot_ore: usize,
    geode_robot_obsidian: usize,
    max_ore: usize,
}

impl Blueprint {
    fn new(s: &str) -> Self {
        let (number, ore_robot_ore, clay_robot_ore, obsidian_robot_ore, obsidian_robot_clay, geode_robot_ore, geode_robot_obsidian) = sscanf::sscanf!(s, "Blueprint {usize}: Each ore robot costs {usize} ore. Each clay robot costs {usize} ore. Each obsidian robot costs {usize} ore and {usize} clay. Each geode robot costs {usize} ore and {usize} obsidian.").expect("blueprint");

        Self {
            number,
            ore_robot_ore,
            clay_robot_ore,
            obsidian_robot_ore,
            obsidian_robot_clay,
            geode_robot_ore,
            geode_robot_obsidian,
            max_ore: ore_robot_ore.max(clay_robot_ore).max(geode_robot_ore),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    time_left: usize,
    ore_robots: usize,
    ore: usize,
    clay_robots: usize,
    clay: usize,
    obsidian_robots: usize,
    obsidian: usize,
    geode_robots: usize,
    geodes: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.geode_robots
            .cmp(&other.geode_robots)
            .then_with(|| self.obsidian_robots.cmp(&other.obsidian_robots))
            .then_with(|| self.clay_robots.cmp(&other.clay_robots))
            .then_with(|| other.time_left.cmp(&self.time_left))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn start(time_left: usize) -> Self {
        Self {
            time_left,
            ore_robots: 1,
            ..Default::default()
        }
    }

    fn neighbours(&self, bp: &Blueprint) -> Vec<Self> {
        let mut next = *self;
        next.time_left -= 1;
        next.ore += next.ore_robots;
        next.clay += next.clay_robots;
        next.obsidian += next.obsidian_robots;
        next.geodes += next.geode_robots;

        let mut neighbours = Vec::new();
        // Wait
        neighbours.push(next);

        // Build a geode robot if we can
        if self.ore >= bp.geode_robot_ore && self.obsidian >= bp.geode_robot_obsidian {
            let mut next = next;
            next.ore -= bp.geode_robot_ore;
            next.obsidian -= bp.geode_robot_obsidian;
            next.geode_robots += 1;
            neighbours.push(next)
        }

        // Build an obsidian robot if we could put them to geode robots, and if we can
        if self.obsidian_robots < bp.geode_robot_obsidian
            && self.ore >= bp.obsidian_robot_ore
            && self.clay >= bp.obsidian_robot_clay
        {
            let mut next = next;
            next.ore -= bp.obsidian_robot_ore;
            next.clay -= bp.obsidian_robot_clay;
            next.obsidian_robots += 1;
            neighbours.push(next)
        }

        // Build a clay robot if we could put them to obsidian robots, and if we can
        if self.clay_robots < bp.obsidian_robot_clay && self.ore >= bp.clay_robot_ore {
            let mut next = next;
            next.ore -= bp.clay_robot_ore;
            next.clay_robots += 1;
            neighbours.push(next);
        }

        // Build an ore robot if we could put the ore to any other robot, and if we can
        if self.ore_robots < bp.max_ore && self.ore >= bp.ore_robot_ore {
            let mut next = next;
            next.ore -= bp.ore_robot_ore;
            next.ore_robots += 1;
            neighbours.push(next);
        }

        neighbours
    }

    fn optimistic_geodes(&self) -> usize {
        let future = if self.time_left == 0 {
            0
        } else {
            self.time_left * (self.time_left - 1) / 2
        };
        self.geodes + self.time_left * self.geode_robots + future
    }
}

#[cfg(test)]
mod state_test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    #[case(1, 0)]
    #[case(2, 1)]
    #[case(3, 3)]
    fn test_optimistic_geodes(#[case] start: usize, #[case] expected: usize) {
        let state = State::start(start);
        assert_eq!(expected, state.optimistic_geodes());
    }
}

impl Blueprint {
    fn score(&self, duration: usize) -> usize {
        let mut seen: HashMap<State, usize> = HashMap::new();
        let mut queue = BinaryHeap::new();
        queue.push(State::start(duration));

        let mut best = 0;
        while let Some(state) = queue.pop() {
            if let Some(score) = seen.get(&state) {
                if *score >= state.geodes {
                    continue;
                }
            }
            seen.insert(state, state.geodes);
            if state.geodes > best {
                best = state.geodes;
            }

            if state.time_left == 0 {
                continue;
            }

            for neighbour in state.neighbours(self) {
                if best >= neighbour.optimistic_geodes() {
                    continue;
                }
                queue.push(neighbour);
            }
        }
        best
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let blueprints: Vec<_> = input.lines().map(Blueprint::new).collect();
    Some(blueprints.iter().map(|bp| bp.number * bp.score(24)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    _ = input;
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blueprint_example_1() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let blueprints: Vec<_> = input.lines().map(Blueprint::new).collect();
        let result = blueprints[0].score(24);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_blueprint_example_2() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let blueprints: Vec<_> = input.lines().map(Blueprint::new).collect();
        let result = blueprints[1].score(24);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
