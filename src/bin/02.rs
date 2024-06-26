advent_of_code::solution!(2);

#[derive(Debug)]
enum Round {
    Win,
    Lose,
    Draw,
}

impl Round {
    fn new(s: &str) -> Self {
        use Round::*;
        match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => unreachable!("bad round char"),
        }
    }

    fn score(&self) -> usize {
        use Round::*;
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Rock,
    Paper,
    Scissors,
}

impl Turn {
    fn new(s: &str) -> Self {
        use Turn::*;
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => unreachable!("bad turn char"),
        }
    }

    fn vs(&self, other: &Self) -> Round {
        use Round::*;
        use Turn::*;
        match (self, other) {
            (Paper, Rock) | (Rock, Scissors) | (Scissors, Paper) => Win,
            (Rock, Paper) | (Scissors, Rock) | (Paper, Scissors) => Lose,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
        }
    }

    fn aiming_to(&self, outcome: &Round) -> Self {
        use Round::*;
        use Turn::*;
        match (outcome, self) {
            (Draw, same) => *same,
            (Win, Scissors) | (Lose, Paper) => Rock,
            (Win, Rock) | (Lose, Scissors) => Paper,
            (Win, Paper) | (Lose, Rock) => Scissors,
        }
    }

    fn score(&self) -> usize {
        use Turn::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| {
                let (them, mine) = l.split_once(' ').unwrap();
                let them = Turn::new(them);
                let mine = Turn::new(mine);
                mine.vs(&them).score() + mine.score()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| {
                let (them, target) = l.split_once(' ').unwrap();
                let them = Turn::new(them);
                let target = Round::new(target);
                let mine = them.aiming_to(&target);
                target.score() + mine.score()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
