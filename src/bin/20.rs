advent_of_code::solution!(20);

fn mix(input: &[i32]) -> Vec<i32> {
    let mut indexes: Vec<usize> = input.iter().enumerate().map(|(i, _)| i).collect();
    for (i, v) in input.iter().enumerate() {
        if *v == 0 {
            continue;
        }
        let pos = indexes.iter().position(|n| *n == i).unwrap();
        let index = indexes.remove(pos);
        let new_pos = (pos as i32 + v).rem_euclid(indexes.len() as i32) as usize;
        if new_pos == 0 {
            indexes.push(index);
        } else {
            indexes.insert(new_pos, index);
        }
    }

    indexes.iter().map(|i| input[*i]).collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let data: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mixed = mix(&data);
    let zero = mixed.iter().position(|&n| n == 0).expect("zero");
    Some(
        (1..=3)
            .map(|i| mixed[(zero + i * 1000) % mixed.len() as usize])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    _ = input;
    None
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
        let result = mix(&input);
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
        assert_eq!(result, None);
    }
}
