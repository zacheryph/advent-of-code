use std::io;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Default)]
struct Square {
  row: i8,
  col: i8,
  num: i8,
  marked: bool,
}

#[derive(Debug)]
struct Board {
  squares: Vec<Square>,
  winning_pick: Option<i8>,
}

impl Board {
  fn won(&self) -> bool {
    self.winning_pick.is_some()
  }

  fn mark(&mut self, num: i8) -> Option<i32> {
    if self.won() { return None }

    if let Some(sq) = self.squares.iter_mut().find(|sq| sq.num == num) {
      sq.marked = true;
    }

    if !self.has_won() { return None }

    self.winning_pick = Some(num);
    self.score()
  }

  fn score(&self) -> Option<i32> {
    match self.winning_pick {
      Some(pick) => Some(pick as i32 * self.unmarked_sum()),
      _ => None,
    }
  }

  fn has_won(&self) -> bool {
    // check rows
    for row in 1..=5 {
      let cnt = self.squares.iter().fold(0, |cnt, sq| {
        if !sq.marked || sq.row != row { return cnt }
        cnt + 1
      });
      if cnt == 5 { return true }
    }
    for col in 1..=5 {
      let cnt = self.squares.iter().fold(0, |cnt, sq| {
        if !sq.marked || sq.col != col { return cnt }
        cnt + 1
      });
      if cnt == 5 { return true }
    }
    return false
  }

  fn unmarked_sum(&self) -> i32 {
    self.squares.iter().fold(0, |acc, sq| {
      if sq.marked { return acc }
      acc + sq.num as i32
    })
  }
}

fn main() -> io::Result<()> {
  let mut lines = INPUT.lines();
  let picks: Vec<i8> = lines.next().unwrap().split(',').map(|x| x.parse::<i8>().unwrap()).collect();
  let mut boards: Vec<Board> = lines.skip(1).collect::<Vec<&str>>().chunks(6).map(|b| {
    let mut squares = Vec::new();
    b.iter().take(5).enumerate().for_each(|(row, line)| {
      line.split_whitespace().enumerate().for_each(|(col, n)| {
        squares.push(Square { row: row as i8+1, col: col as i8+1, num: n.parse::<i8>().unwrap(), marked: false });
      });
    });

    Board { squares, winning_pick: None }
  }).collect();

  let stages = picks.iter().fold((None, None), |mut acc, &pick| {
    boards.iter_mut().for_each(|board| {
      if let Some(win) = board.mark(pick) {
        acc.0 = acc.0.or(Some(win));
        acc.1 = Some(win);
      }
    });
    acc
  });

  // Stage 1: 12796
  println!("Stage 1: {}", stages.0.unwrap_or(0));
  // Stage 2: 18063
  println!("Stage 2: {}", stages.1.unwrap_or(0));

  Ok(())
}
