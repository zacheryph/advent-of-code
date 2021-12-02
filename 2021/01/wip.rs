use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() -> io::Result<()> {
    // Stage 1: 1532
    let lines = read_lines(Path::new("input.txt"))?;
    let mut result: i32 = -1;
    let mut current: i32 = 0;
    for l in lines {
        let num: i32 = l.unwrap().parse().unwrap();
        if num > current {
            result += 1
        }
        current = num;
    }
    println!("Stage 1: {}", result);

    // Stage 2: 1571
    // let lines = read_lines(Path::new("test.txt"))?;
    let mut lines = read_lines(Path::new("input.txt"))?;
    let mut result: i32 = 0;
    let mut tail: Vec<i32> = Vec::new();
    lines.by_ref().take(3).for_each(|l| {
        let num: i32 = l.unwrap().parse().unwrap();
        tail.push(num);
    });

    for l in lines {
        tail.push(l.unwrap().parse().unwrap());
        let front = tail[0..=2].into_iter().fold(0, |m, &x| m + x);
        let back = tail[1..=3].into_iter().fold(0, |m, &x| m + x);
        if back > front {
            result += 1
        }
        // println!("F:{} B:{} T:{:?} => {}", front, back, tail, back > front);
        tail = tail.split_off(1);
    }
    println!("Stage 2: {}", result);

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
