use std::str::FromStr;

pub fn part_one(input: &str) -> Option<i32> {
    let position = input.lines().map(|l| l.parse::<Movement>().unwrap()).fold(
        Position::default(),
        |mut pos, movement| {
            match movement {
                Movement {
                    direction: Direction::Up,
                    units,
                } => pos.depth -= units,
                Movement {
                    direction: Direction::Down,
                    units,
                } => pos.depth += units,
                Movement {
                    direction: Direction::Forward,
                    units,
                } => pos.advance += units,
            }
            pos
        },
    );

    Some(position.result())
}

pub fn part_two(input: &str) -> Option<i32> {
    let position = input.lines().map(|l| l.parse::<Movement>().unwrap()).fold(
        Position::default(),
        |mut pos, movement| {
            match movement {
                Movement {
                    direction: Direction::Up,
                    units,
                } => pos.aim -= units,
                Movement {
                    direction: Direction::Down,
                    units,
                } => pos.aim += units,
                Movement {
                    direction: Direction::Forward,
                    units,
                } => {
                    pos.advance += units;
                    pos.depth += pos.aim * units;
                }
            }
            pos
        },
    );

    Some(position.result())
}

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    units: i32,
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = input.split(' ').collect();
        if split.len() != 2 {
            return Err(());
        }
        Ok(Self {
            direction: split[0].parse().unwrap(),
            units: split[1].parse().unwrap(),
        })
    }
}

#[derive(Default, Debug)]
struct Position {
    advance: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn result(&self) -> i32 {
        self.advance * self.depth
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
