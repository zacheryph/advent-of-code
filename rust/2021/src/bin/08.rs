use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let appearances = input.lines().fold(0, |acc, l| {
        let mut pieces = l
            .split('|')
            .map(|l| l.trim().split_whitespace().collect())
            .take(2);
        let _: Vec<&str> = pieces.next().unwrap();
        let output: Vec<&str> = pieces.next().unwrap();
        acc + output.iter().fold(0, |acc, n| match n.len() {
            2 | 3 | 4 | 7 => acc + 1,
            _ => acc,
        })
    });

    Some(appearances)
}

pub fn part_two(input: &str) -> Option<i32> {
    let total = input
        .lines()
        .map(|l| {
            let (input, output) = l.split_once("|").unwrap();
            let digits = Signal::map(input);

            let output: i32 = output
                .split_whitespace()
                .map(|sig| Signal::new(sig).unwrap())
                .map(|sig| digits.get(&sig.wires).unwrap())
                .fold(0 as i32, |acc, numeral| acc * 10 + *numeral as i32);

            output
        })
        .fold(0, |acc, n| acc + n);

    Some(total)
}

fn wire_bits() -> HashMap<char, u8> {
    HashMap::from([
        ('a', 0x01),
        ('b', 0x02),
        ('c', 0x04),
        ('d', 0x08),
        ('e', 0x10),
        ('f', 0x20),
        ('g', 0x40),
    ])
}

#[derive(Debug)]
struct Signal {
    len: u8,
    wires: u8,
}

fn pull_digit<T>(signals: &mut Vec<Signal>, seeker: T) -> u8
where
    T: Fn(&Signal) -> bool,
{
    let (idx, sig) = signals
        .iter()
        .enumerate()
        .find(|(_, sig)| seeker(&sig))
        .unwrap();
    let res = sig.wires;
    signals.swap_remove(idx);
    res
}

impl Signal {
    fn new(input: &str) -> Result<Self, &str> {
        let wire_map = wire_bits();
        let wires = input
            .chars()
            .fold(0, |acc, l| acc | *wire_map.get(&l).unwrap());

        Ok(Self {
            len: input.len() as u8,
            wires,
        })
    }

    fn map(input: &str) -> HashMap<u8, u8> {
        let mut input: Vec<Signal> = input
            .split_whitespace()
            .map(|sig| Signal::new(sig).unwrap())
            .collect();
        let mut res = vec![0; 10];

        res[1] = pull_digit(&mut input, |sig| sig.len == 2);
        res[4] = pull_digit(&mut input, |sig| sig.len == 4);
        res[7] = pull_digit(&mut input, |sig| sig.len == 3);
        res[8] = pull_digit(&mut input, |sig| sig.len == 7);
        res[9] = pull_digit(&mut input, |sig| sig.wires & res[4] == res[4]);
        res[0] = pull_digit(&mut input, |sig| {
            sig.len == 6 && sig.wires & res[7] == res[7]
        });
        res[6] = pull_digit(&mut input, |sig| sig.len == 6);
        res[3] = pull_digit(&mut input, |sig| sig.wires & res[1] == res[1]);
        res[2] = pull_digit(&mut input, |sig| {
            sig.wires & res[6] & !res[9] == res[6] & !res[9]
        });
        res[5] = pull_digit(&mut input, |_| true);

        let mut newres: HashMap<u8, u8> = HashMap::new();
        res.iter().enumerate().for_each(|(idx, wires)| {
            newres.insert(*wires, idx as u8);
        });

        newres
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
