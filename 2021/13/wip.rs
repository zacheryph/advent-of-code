use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");

#[derive(Debug)]
struct Paper {
    dots: HashSet<(i64, i64)>,
    folds: Vec<(char, i64)>,
    rows: i64,
    cols: i64,
}

impl Paper {
    fn new() -> Self {
        let mut dots: HashSet<(i64, i64)> = HashSet::new();
        let mut folds: Vec<(char, i64)> = Vec::new();
        let mut rows: i64 = 0;
        let mut cols: i64 = 0;

        INPUT.lines().for_each(|l| {
            if "".eq(l.trim()) {
                return;
            }

            match l.strip_prefix("fold along ") {
                Some(f) => {
                    let (c, l) = f.split_once("=").unwrap();
                    folds.push((c.chars().next().unwrap(), l.parse::<i64>().unwrap()));
                }
                None => {
                    let (c, r) = l.split_once(",").unwrap();
                    let c = c.parse::<i64>().unwrap();
                    let r = r.parse::<i64>().unwrap();
                    cols = std::cmp::max(cols, c);
                    rows = std::cmp::max(rows, r);
                    dots.insert((c, r));
                }
            }
        });

        Self {
            dots,
            folds,
            rows,
            cols,
        }
    }

    fn fold(&mut self, fold: (char, i64)) {
        let mut dots: HashSet<(i64, i64)> = HashSet::new();

        if fold.0 == 'y' {
            let row = fold.1;
            self.rows = row - 1;

            self.dots.iter().for_each(|(c, r)| {
                if *r < row {
                    dots.insert((*c, *r));
                } else {
                    dots.insert((*c, row - (*r - row)));
                }
            })
        }

        if fold.0 == 'x' {
            let col = fold.1;
            self.cols = col - 1;

            self.dots.iter().for_each(|(c, r)| {
                if *c < col {
                    dots.insert((*c, *r));
                } else {
                    dots.insert((col - (*c - col), *r));
                }
            })
        }

        self.dots = dots;
    }
}

fn stage_one() -> i64 {
    let mut paper = Paper::new();
    paper.fold(paper.folds[0]);
    paper.dots.len() as i64
}

fn stage_two() {
    let mut paper = Paper::new();
    for i in 0..paper.folds.len() {
        paper.fold(paper.folds[i]);
    }

    for r in 0..=paper.rows as i64 {
        for c in 0..=paper.cols as i64 {
            if paper.dots.contains(&(c, r)) {
                print!("#")
            } else {
                print!(" ")
            }
        }
        println!();
    }
}

fn main() {
    // Stage 1:
    println!("Stage 1: {}", stage_one());
    // Stage 2:
    println!("Stage 2:");
    stage_two();
}
