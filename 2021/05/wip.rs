use std::collections::HashMap;
use std::io;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Point {
  x: i16,
  y: i16,
}

impl FromStr for Point {
  type Err = ();

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let numbers: Vec<i16> = input
      .split(',')
      .take(2)
      .map(|n| n.parse().unwrap())
      .collect();
    if numbers.len() != 2 {
      return Err(());
    }
    Ok(Self { x: numbers[0], y: numbers[1] })
  }
}

type LineFn = fn(Point, Point) -> Option<Vec<Point>>;

fn avoid_count(input: &str, setter: LineFn) -> i32 {
  let mut stage: HashMap<Point, i16> = HashMap::new();
  input.lines().for_each(|l| {
    let mut points = l.split(" -> ").map(|p| p.parse::<Point>().unwrap());
    if let Some(points) = setter(points.next().unwrap(), points.next().unwrap()) {
      points.iter().for_each(|p| {
        let cnt = match stage.get(p) {
          Some(c) => c + 1,
          None => 1
        };
        stage.insert(*p, cnt);
      });
    }
  });
  stage.iter().fold(0, |acc, (_, v)| if *v > 1 { acc + 1 } else { acc })
}

fn stage_one_line(a: Point, b: Point) -> Option<Vec<Point>> {
  if a.x == b.x {
    let range = if a.y < b.y { a.y..=b.y } else { b.y..=a.y };
    return Some(range.map(|y| Point { x: a.x, y }).collect());
  }
  if a.y == b.y {
    let range = if a.x < b.x { a.x..=b.x } else { b.x..=a.x };
    return Some(range.map(|x| Point { x, y: a.y }).collect());
  }

  None
}

fn stage_two_line(a: Point, b: Point) -> Option<Vec<Point>> {
  if let Some(line) = stage_one_line(a, b) { return Some(line) }

  let x: Vec<i16> = if a.x < b.x { (a.x..=b.x).collect() } else { (b.x..=a.x).rev().collect() };
  let y: Vec<i16> = if a.y < b.y { (a.y..=b.y).collect() } else { (b.y..=a.y).rev().collect() };
  if x.len() != y.len() { return None }

  Some(x.iter().zip(y.iter()).map(|(x, y)| Point { x: *x, y: *y }).collect())
}

fn main() -> io::Result<()> {
  // Stage 1: 7414
  let avoid = avoid_count(INPUT, stage_one_line);
  println!("Stage 1: {}", avoid);

  // Stage 2: 19676
  let avoid = avoid_count(INPUT, stage_two_line);
  println!("Stage 2: {}", avoid);

  Ok(())
}
