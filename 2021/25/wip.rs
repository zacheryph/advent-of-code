#![allow(dead_code)]
use std::collections::HashMap;

// const INPUT: &str = include_str!("input.txt");
const INPUT: &str = include_str!("test.txt");

type Point = (usize, usize);

#[derive(Debug)]
enum Direction {
    Right,
    Down,
}

#[derive(Debug)]
struct SeaFloor {
    cucumbers: HashMap<Point, Direction>,
    rows: usize,
    cols: usize,
}

fn parse_input() -> SeaFloor {
    let mut cucumbers: HashMap<Point, Direction> = HashMap::new();
    let mut rows = 0;
    let mut cols = 0;

    INPUT.lines().enumerate().for_each(|(row, l)| {
        rows = std::cmp::max(rows, row);

        l.chars().enumerate().for_each(|(col, c)| {
            cols = std::cmp::max(cols, col);

            match c {
                '>' => { cucumbers.insert((row, col), Direction::Right); },
                'v' => { cucumbers.insert((row, col), Direction::Down); },
                _ => {},
            }
        })
    });

    SeaFloor { cucumbers, rows, cols }
}

impl SeaFloor {
    fn step(&mut self) -> usize {
        for row in (0..=self.rows).rev() {
            for col in (0..=self.cols).rev() {

            }
        }

        0
    }
}

fn stage_one() -> i64 {
    let _floor = parse_input();

    for i in (0..=5).rev() {
        println!("WOOT {}", i);
    }

    0
}

fn stage_two() -> i64 {
    0
}

fn main() {
    // Stage 1:
    println!("Stage 1: {}", stage_one());
    // Stage 2:
    println!("Stage 2: {}", stage_two());
}
