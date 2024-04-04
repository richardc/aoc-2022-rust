advent_of_code::solution!(20);

fn mix(input: &[i32]) -> Vec<i32> {
    input.to_vec()
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
