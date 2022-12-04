pub fn part_one(input: &str) -> Option<i32> {
    let mut result = -1;
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .fold(0, |prev, n| {
            if n > prev {
                result += 1
            }
            n
        });
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut depths = input.lines().map(|l| l.parse().unwrap());
    let mut tail: Vec<i32> = depths.by_ref().take(3).collect();
    let result = depths.fold(0, |acc, n: i32| {
        tail.push(n);
        let front = tail[0..=2].into_iter().fold(0, |m, &x| m + x);
        let back = tail[1..=3].into_iter().fold(0, |m, &x| m + x);
        tail = tail.split_off(1);

        if back > front {
            acc + 1
        } else {
            acc
        }
    });
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
