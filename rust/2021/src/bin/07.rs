pub fn part_one(input: &str) -> Option<i32> {
    let numbers = input_numbers(input);
    let median = numbers[numbers.len() / 2];
    let fuel = stage_one_fuel(&numbers, median);
    Some(fuel)
}

pub fn part_two(input: &str) -> Option<i32> {
    let numbers = input_numbers(input);
    let average: i32 = (numbers.iter().sum::<i32>() as f32 / numbers.len() as f32).round() as i32;

    // Some mad scientist from _the internet_ claims the
    // number we really want _will_ be within one of the
    // mean. I'm not a mad scientist so... I trust them.
    // So we grab within 2 and take the smallest.
    //
    // truth be told, i found this after solving, i knew
    // the answer _should?_ be the mean (because im vaguely
    // smart... i guess), i ended up bisecting between the
    // median and average to find it being within one of mean.
    let range: Vec<i32> = ((average - 2)..=(average + 2))
        .map(|n| stage_two_fuel(&numbers, n))
        .collect();
    let fuel = range.iter().fold(i32::MAX, |acc, &n| i32::min(acc, n));

    Some(fuel)
}

fn stage_one_fuel(numbers: &Vec<i32>, stop: i32) -> i32 {
    numbers.iter().fold(0, |acc, n| (acc + (n - stop).abs()))
}

fn stage_two_fuel(numbers: &Vec<i32>, stop: i32) -> i32 {
    numbers
        .iter()
        .fold(0, |acc, n| acc + (1..=(n - stop).abs()).sum::<i32>())
}

fn input_numbers(input: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = input
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    numbers.sort();
    numbers
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
