const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");

struct Stage {
  rows: usize,
  cols: usize,
  stage: Vec<Vec<u32>>,
}

impl Stage {
  fn fill() -> Self {
    let stage: Vec<Vec<u32>> = INPUT.lines().map(|l| {
      l.chars().map(|c| c.to_digit(10).unwrap() as u32).collect()
    }).collect();

    Self { rows: stage.len() - 1, cols: stage[0].len() - 1, stage }
  }

  fn get(&self, row: usize, col: usize) -> u32 {
    self.stage[row][col]
  }

  fn mark(&mut self, row: usize, col: usize) {
    self.stage[row][col] = 9;
  }
}

fn stage_one() -> u32 {
  let stage = Stage::fill();

  let mut low_spots: Vec<u32> = Vec::new();
  for row in 0..=stage.rows {
    for col in 0..=stage.cols {
      let current = stage.get(row, col);
      let above = if row > 0 { stage.get(row-1, col) } else { 9 };
      let below = if row < stage.rows { stage.get(row+1, col) } else { 9 };
      let left = if col > 0 { stage.get(row, col-1) } else { 9 };
      let right = if col < stage.cols { stage.get(row, col+1) } else { 9 };

      if above <= current || below <= current || left <= current || right <= current { continue }
      low_spots.push(current);
    }
  }

  low_spots.iter().fold(0, |acc, spot| acc + spot + 1)
}


fn walk_basin(stage: &mut Stage, row: usize, col: usize) -> u32 {
  if stage.get(row, col) == 9 { return 0 }
  stage.mark(row, col);

  let above = if row > 0 { walk_basin(stage, row-1, col) } else { 0 };
  let below = if row < stage.rows { walk_basin(stage, row+1, col) } else { 0 };
  let left = if col > 0 { walk_basin(stage, row, col-1) } else { 0 };
  let right = if col < stage.cols { walk_basin(stage, row, col+1) } else { 0 };

  1 + above + below + left + right
}

fn stage_two() -> u32 {
  let mut basin_sizes: Vec<u32> = Vec::new();
  let mut stage = Stage::fill();

  for row in 0..=stage.rows {
    for col in 0..=stage.cols {
      if stage.get(row, col) == 9 { continue }

      basin_sizes.push(walk_basin(&mut stage, row, col));
    }
  }

  basin_sizes.sort_by(|a, b| b.cmp(a));
  basin_sizes.iter().take(3).fold(1, |acc, basin| acc * basin)
}

fn main() -> std::io::Result<()> {
  // Stage 1: 448
  println!("Stage 1: {}", stage_one());
  // Stage 2: 1417248
  println!("Stage 2: {}", stage_two());

  Ok(())
}
