advent_of_code::solution!(15);

use range_set_blaze::RangeSetBlaze;
use winnow::{ascii::dec_int, bytes::tag, IResult, Parser};

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
    let (input, (_, x1, _, y1, _, x2, _, y2)) = (
        tag("Sensor at x="),
        dec_int,
        tag(", y="),
        dec_int,
        tag(": closest beacon is at x="),
        dec_int,
        tag(", y="),
        dec_int,
    )
        .parse_next(input)?;

    Ok((input, (Point(x1, y1), Point(x2, y2))))
}

impl Sensors {
    fn new(s: &str) -> Self {
        let mut sensors: Vec<(Point, Point)> = s.lines().map(|l| sensor(l).unwrap().1).collect();
        sensors.sort_by_key(|s| s.0 .0);
        Self { sensors }
    }

    fn scanned_xs(&self, y: i32) -> RangeSetBlaze<i32> {
        let mut scanned: RangeSetBlaze<i32> = RangeSetBlaze::new();
        for (sensor, beacon) in &self.sensors {
            let distance = sensor.manhattan(beacon);
            let intersect = sensor.1 - y;
            if intersect.unsigned_abs() > distance {
                // No overlap
                continue;
            }

            let x_min = sensor.0 - (distance as i32 - intersect.abs()).abs();
            let x_max = sensor.0 + (distance as i32 - intersect.abs()).abs();
            scanned.ranges_insert(x_min..=x_max);
        }
        scanned
    }

    fn excluded_cells(&self, y: i32) -> usize {
        let mut excluded = self.scanned_xs(y);

        for (_, beacon) in &self.sensors {
            if beacon.1 == y {
                excluded.remove(beacon.0);
            }
        }

        excluded.len()
    }

    fn uncovered_slot_frequency(&self, dim: i32) -> usize {
        for y in 0..dim {
            let scanned = self.scanned_xs(y);
            if scanned.ranges_len() == 2 {
                let x = scanned.ranges().next().unwrap().max().unwrap() + 1;
                return x as usize * 4000000 + y as usize;
            }
        }
        0
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let sensors = Sensors::new(input);
    Some(sensors.excluded_cells(2_000_000))
}

pub fn part_two(input: &str) -> Option<usize> {
    let sensors = Sensors::new(input);
    Some(sensors.uncovered_slot_frequency(4_000_000))
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
        let sensors = Sensors::new(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(sensors.uncovered_slot_frequency(20), 56_000_011);
    }
}
