advent_of_code::solution!(20);

fn mix(input: &[i64], rounds: usize) -> Vec<i64> {
    let mut indexes: Vec<usize> = input.iter().enumerate().map(|(i, _)| i).collect();
    for _ in 0..rounds {
        for (i, v) in input.iter().enumerate() {
            if *v == 0 {
                continue;
            }
            let pos = indexes.iter().position(|n| *n == i).unwrap();
            let index = indexes.remove(pos);
            let new_pos = (pos as i64 + v).rem_euclid(indexes.len() as i64) as usize;
            if new_pos == 0 {
                indexes.push(index);
            } else {
                indexes.insert(new_pos, index);
            }
        }
    }

    indexes.iter().map(|i| input[*i]).collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let data: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mixed = mix(&data, 1);
    let zero = mixed.iter().position(|&n| n == 0).expect("zero");
    Some(
        (1..=3)
            .map(|i| mixed[(zero + i * 1000) % mixed.len() as usize])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let data: Vec<_> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * 811589153)
        .collect();
    let mixed = mix(&data, 10);
    let zero = mixed.iter().position(|&n| n == 0).expect("zero");
    Some(
        (1..=3)
            .map(|i| mixed[(zero + i * 1000) % mixed.len() as usize])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let input: Vec<_> = advent_of_code::template::read_file("examples", DAY)
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        let result = mix(&input, 1);
        assert_eq!(result, [1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1623178306));
    }
}
