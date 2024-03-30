advent_of_code::solution!(15);

use nom::{bytes::complete::tag, character::complete::i32, sequence::tuple, IResult};
use std::collections::HashSet;

#[derive(Debug)]
struct Point(i32, i32);

impl Point {
    fn manhattan(&self, other: &Self) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug)]
struct Sensors {
    sensors: Vec<(Point, Point)>,
}

fn sensor(input: &str) -> IResult<&str, (Point, Point)> {
    let (input, (_, x1, _, y1, _, x2, _, y2)) = tuple((
        tag("Sensor at x="),
        i32,
        tag(", y="),
        i32,
        tag(": closest beacon is at x="),
        i32,
        tag(", y="),
        i32,
    ))(input)?;

    Ok((input, (Point(x1, y1), Point(x2, y2))))
}

impl Sensors {
    fn new(s: &str) -> Self {
        let sensors = s.lines().map(|l| sensor(l).unwrap().1).collect();
        Self { sensors }
    }

    fn excluded_cells(&self, y: i32) -> usize {
        let mut excluded: HashSet<i32> = HashSet::new();
        for (sensor, beacon) in &self.sensors {
            let distance = sensor.manhattan(&beacon);
            let intersect = sensor.1 - y;
            if intersect.unsigned_abs() > distance {
                // No overlap
                continue;
            }

            let x_min = sensor.0 - (distance as i32 - intersect.abs()).abs();
            let x_max = sensor.0 + (distance as i32 - intersect.abs()).abs();
            excluded.extend(x_min..=x_max);
        }

        for (_, beacon) in &self.sensors {
            if beacon.1 == y {
                excluded.remove(&beacon.0);
            }
        }

        excluded.len()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let sensors = Sensors::new(input);
    Some(sensors.excluded_cells(2_000_000))
}

pub fn part_two(input: &str) -> Option<u32> {
    let _sensors = Sensors::new(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let sensors = Sensors::new(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(sensors.excluded_cells(10), 26);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
