use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;

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

fn main() -> io::Result<()> {
    // Stage 1: 1636725
    let lines = read_lines(Path::new("input.txt"))?;
    let mut position = 0;
    let mut depth = 0;

    for l in lines {
        let movement = l.unwrap().parse::<Movement>().unwrap();

        match movement {
            Movement {
                direction: Direction::Up,
                units,
            } => depth -= units,
            Movement {
                direction: Direction::Down,
                units,
            } => depth += units,
            Movement {
                direction: Direction::Forward,
                units,
            } => position += units,
        }
    }

    println!(
        "Stage 1: Postion:{} Depth:{} Result:{}",
        position,
        depth,
        position * depth
    );

    // Stage 2: 1872757425
    let lines = read_lines(Path::new("input.txt"))?;
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for l in lines {
        let movement = l.unwrap().parse::<Movement>().unwrap();

        match movement {
            Movement {
                direction: Direction::Up,
                units,
            } => aim -= units,
            Movement {
                direction: Direction::Down,
                units,
            } => aim += units,
            Movement {
                direction: Direction::Forward,
                units,
            } => {
                position += units;
                depth += aim * units;
            }
        }
    }

    println!(
        "Stage 1: Postion:{} Depth:{} Aim:{}, Result:{}",
        position,
        depth,
        aim,
        position * depth
    );

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
