use std::io;

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

fn stage_one_fuel(numbers: &Vec<i32>, stop: i32) -> i32 {
  numbers.iter().fold(0, |acc, n| (acc + (n - stop).abs()))
}

fn stage_two_fuel(numbers: &Vec<i32>, stop: i32) -> i32 {
  numbers.iter().fold(0, |acc, n| {
    acc + (1..=(n - stop).abs()).sum::<i32>()
  })
}

fn main() -> io::Result<()> {
  let mut numbers: Vec<i32> = INPUT.split(',').map(|n| n.trim().parse().unwrap()).collect();
  numbers.sort();

  // Stage 1: 335330
  let median = numbers[numbers.len() / 2];
  let fuel = stage_one_fuel(&numbers, median);
  println!("Stage 1: {} (median: {})", fuel, median);

  // Stage 2: 92439766
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
  let range: Vec<i32> = ((average -2)..=(average+2)).map(|n| stage_two_fuel(&numbers, n)).collect();
  let fuel = range.iter().fold(i32::MAX, |acc, &n| i32::min(acc, n));
  println!("Stage 2: {}", fuel);

  Ok(())
}
