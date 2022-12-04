use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i64> {
    let mut stage = Stage::new(input);
    let mut total_flashes: i64 = 0;

    for _ in 0..100 {
        stage.increase();
        total_flashes += stage.flash();
    }

    Some(total_flashes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stage = Stage::new(input);
    let total = stage.stage.len() as i64;

    for cycle in 1.. {
        stage.increase();
        if total == stage.flash() {
            return Some(cycle);
        }
    }
    None
}

const OFFSETS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

struct Stage {
    rows: i64,
    cols: i64,
    stage: HashMap<(i64, i64), i64>,
}

fn adjacent_points(point: &(i64, i64)) -> [(i64, i64); 8] {
    OFFSETS.map(|(l, c)| (point.0 + l, point.1 + c))
}

impl Stage {
    fn new(input: &str) -> Self {
        let mut stage: HashMap<(i64, i64), i64> = HashMap::new();
        let mut rows = 0;

        input.lines().enumerate().for_each(|(line_no, l)| {
            rows += 1;
            l.chars().enumerate().for_each(|(idx, c)| {
                stage.insert((line_no as i64, idx as i64), c.to_digit(10).unwrap() as i64);
            });
        });

        Self {
            rows,
            cols: stage.len() as i64 / rows,
            stage,
        }
    }

    fn increase(&mut self) {
        self.stage.iter_mut().for_each(|(_, v)| *v += 1);
    }

    fn flash(&mut self) -> i64 {
        let mut flashes: i64 = 0;
        let mut increase: Vec<(i64, i64)> = Vec::new();

        loop {
            self.stage
                .iter_mut()
                .filter(|(_, energy)| **energy > 9)
                .for_each(|(point, energy)| {
                    flashes += 1;
                    *energy = 0;
                    adjacent_points(point).iter().for_each(|p| {
                        increase.push(*p);
                    });
                });

            if increase.len() == 0 {
                break;
            }

            increase
                .drain(0..)
                .for_each(|p| match self.stage.get_mut(&p) {
                    None => {}
                    Some(0) => {}
                    Some(n) => *n += 1,
                })
        }

        flashes
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Current Stage:");
        for r in 0..self.rows {
            for c in 0..self.cols {
                print!("{:02} ", self.stage.get(&(r, c)).unwrap());
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
