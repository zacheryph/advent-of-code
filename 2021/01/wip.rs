use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn main() -> io::Result<()> {
    let input_path = Path::new("input.txt");

    // Stage 1 (1532): Take 2
    let mut result = -1;
    read_lines(input_path)?
        .map(|l| l.unwrap().parse().unwrap())
        .fold(0, |prev, n| {
            if n > prev { result += 1 }
            n
        });

    println!("Stage 1: {}", result);

    // Stage 2 (1571): Take 2
    let mut depths = read_lines(input_path)?.map(|l| l.unwrap().parse().unwrap());
    let mut tail: Vec<i32> = depths.by_ref().take(3).collect();
    let result: i32 = depths.fold(0, |acc, n: i32| {
        tail.push(n);
        let front = tail[0..=2].into_iter().fold(0, |m, &x| m + x);
        let back = tail[1..=3].into_iter().fold(0, |m, &x| m + x);
        tail = tail.split_off(1);

        if back > front { acc + 1 } else { acc }
    });

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
