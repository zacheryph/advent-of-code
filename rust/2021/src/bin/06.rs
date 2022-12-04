pub fn part_one(input: &str) -> Option<u64> {
    Some(lifespan_stages(input, 80))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(lifespan_stages(input, 256))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn lifespan_stages(input: &str, iterations: i32) -> u64 {
    let mut lifespans: Vec<u64> = input.split(',').map(|n| n.trim().parse().unwrap()).fold(
        vec![0 as u64; 9],
        |mut acc, n: usize| {
            acc[n] += 1;
            acc
        },
    );

    for _ in 0..iterations {
        lifespans =
            lifespans
                .iter()
                .enumerate()
                .fold(vec![0 as u64; 9], |mut acc, (days, units)| {
                    match days {
                        1.. => acc[days - 1] += units,
                        0 => {
                            acc[6] += units;
                            acc[8] += units;
                        }
                        _ => panic!(),
                    }
                    acc
                })
    }

    lifespans.iter().fold(0, |acc, v| acc + v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
