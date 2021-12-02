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

fn main() -> io::Result<()> {
    let input_path = Path::new("input.txt");

    // Stage 1 (1636725): Take 2
    let position = read_lines(input_path)?
        .map(|l| l.unwrap().parse::<Movement>().unwrap())
        .fold(Position::default(), |mut pos, movement| {
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
        });

    println!("Stage 1: {:?} => {}", position, position.result());

    // Stage 2 (1872757425): Take 2
    let position = read_lines(input_path)?
        .map(|l| l.unwrap().parse::<Movement>().unwrap())
        .fold(Position::default(), |mut pos, movement| {
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
        });

    println!("Stage 2: {:?} => {}", position, position.result());

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
