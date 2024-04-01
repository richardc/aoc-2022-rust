use std::fmt::Display;

advent_of_code::solution!(17);

#[rustfmt::skip] // We laid these bit patterns out just so.
const ROCKS: [u32; 5] = [
    // Line
      0b00111100,
    // Cross +
    ((0b00010000 << 16) +
     (0b00111000 << 8) +
      0b00010000),
    // J
    ((0b00001000 << 16) +
     (0b00001000 << 8) +
      0b00111000),
    // Line |
    ((0b00100000 << 24) +
     (0b00100000 << 16) +
     (0b00100000 << 8) +
     0b00100000),
    // Square
    ((0b00110000 << 8) +
      0b00110000),
];

#[derive(Debug, Default)]
struct Well {
    rock: u32,
    y: i32,
    directions: Vec<char>,
    direction: usize,
    next_rock: usize,
    rocks_placed: usize,
    well: Vec<u8>,
    height: i32,
}

impl Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.well.len()).rev() {
            let well = self.well[y];
            let piece = match y as i32 - self.y {
                3 => self.rock >> 24,
                2 => self.rock >> 16,
                1 => self.rock >> 8,
                0 => self.rock,
                _ => 0,
            };

            write!(f, "|")?;
            for i in (1..8).rev() {
                let piece_bit = ((piece >> i) & 1) == 1;
                let well_bit = ((well >> i) & 1) == 1;
                write!(
                    f,
                    "{}",
                    if piece_bit {
                        '@'
                    } else if well_bit {
                        '#'
                    } else {
                        ' '
                    }
                )?;
            }
            write!(f, "|")?;
            if y == self.y as usize {
                write!(f, "  <- y = {y}")?;
            }
            if y == self.height as usize {
                write!(f, "  <- height = {y}")?;
            }
            writeln!(f)?;
        }
        writeln!(f, "+0123456+")
    }
}

impl Well {
    fn new(s: &str) -> Self {
        let directions = s.trim().chars().collect();

        Self {
            directions,
            y: 3,
            rock: ROCKS[0],
            next_rock: 1,
            well: vec![0; 10],
            ..Default::default()
        }
    }

    fn well_bits(&self, y: i32) -> u32 {
        const WALL: u32 = 0b00000001;
        (((self.well[(y + 3) as usize] as u32) | WALL) << 24)
            + (((self.well[(y + 2) as usize] as u32) | WALL) << 16)
            + (((self.well[(y + 1) as usize] as u32) | WALL) << 8)
            + ((self.well[y as usize] as u32) | WALL)
    }

    fn step(&mut self) {
        let push = self.directions[self.direction];
        self.direction = (self.direction + 1) % self.directions.len();

        let well_bits = self.well_bits(self.y);
        match push {
            '<' => {
                if ((self.rock << 1) & well_bits) == 0 {
                    self.rock <<= 1;
                }
            }
            '>' => {
                if ((self.rock >> 1) & well_bits) == 0 {
                    self.rock >>= 1;
                }
            }
            _ => unreachable!("only have <>"),
        }

        if self.y > 0 && self.rock & self.well_bits(self.y - 1) == 0 {
            self.y -= 1;
            return;
        }

        // blit rock into well, tracking height of this block
        let mut block_height = 0;
        for offset in 0..4 {
            let bits = ((self.rock >> offset * 8) & 0xFF) as u8;
            if bits != 0 {
                block_height = block_height.max(offset + self.y);
                self.well[(self.y + offset) as usize] |= bits;
            }
        }

        self.height = self.height.max(block_height + 1);

        //  Grow the well to accomodate next block
        let well_space = (self.height + 3 + 4) as usize;
        if self.well.len() < well_space {
            let grow = well_space - self.well.len();
            self.well.extend(vec![0; grow]);
        }

        self.rocks_placed += 1;

        // next rock
        self.rock = ROCKS[self.next_rock];
        self.next_rock = (self.next_rock + 1) % ROCKS.len();
        self.y = self.height + 3;
    }
}

#[allow(dead_code)]
fn printy_drop_blocks(input: &str, count: usize) -> Well {
    let mut well = Well::new(input);
    let mut last = well.rocks_placed;
    println!("{}", well);
    while well.rocks_placed < count {
        well.step();
        if last != well.rocks_placed {
            last = well.rocks_placed;
            println!("{}", well);
        }
    }
    well
}

fn drop_blocks(input: &str, count: usize) -> Well {
    let mut well = Well::new(input);
    while well.rocks_placed < count {
        well.step();
    }
    well
}

pub fn part_one(input: &str) -> Option<i32> {
    let well = drop_blocks(input, 2022);
    Some(well.height)
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
        assert_eq!(result, Some(3068));
    }

    #[test]
    fn test_first_rock() {
        let well = drop_blocks(&advent_of_code::template::read_file("examples", DAY), 1);
        assert_eq!(well.height, 1);
    }

    #[test]
    fn test_second_rock() {
        let well = drop_blocks(&advent_of_code::template::read_file("examples", DAY), 2);
        assert_eq!(well.height, 4);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
