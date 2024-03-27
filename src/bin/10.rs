advent_of_code::solution!(10);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let program = input
        .lines()
        .flat_map(|l| {
            if let Some(value) = l.strip_prefix("addx ") {
                let value = value.parse::<i32>().expect("to be a number");
                vec![0, value]
            } else {
                vec![0]
            }
        })
        .collect_vec();

    let mut x = 1;
    let mut total = 0;
    for (i, v) in (1..=220).zip(program) {
        if (i - 20) % 40 == 0 {
            total += i * x;
        }
        x += v;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<String> {
    let program = input
        .lines()
        .flat_map(|l| {
            if let Some(value) = l.strip_prefix("addx ") {
                let value = value.parse::<i32>().expect("to be a number");
                vec![0, value]
            } else {
                vec![0]
            }
        })
        .collect_vec();

    let mut x: i32 = 1;
    let mut crt = [false; 240];
    for (i, v) in (1..=240).zip(program) {
        let pixel = ((i - 1) % 40) + 1;
        let sprite = ((x - 1) % 40) + 1;
        if pixel >= sprite && pixel <= sprite + 2 {
            crt[(i - 1) as usize] = true;
        }
        x += v;
    }

    let display = crt
        .iter()
        .map(|&x| if x { b'#' } else { b' ' })
        .collect_vec()
        .chunks(40)
        .map(|scanline| String::from_utf8_lossy(scanline).into_owned())
        .join("\n");
    Some(display)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let picture = &advent_of_code::template::read_file_part("examples", DAY, 2);
        assert_eq!(result, Some(picture.replace('.', " ").to_string()));
    }
}
