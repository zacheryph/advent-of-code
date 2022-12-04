pub fn part_one(input: &str) -> Option<usize> {
    let places = parse_input(input);
    let mut places = (start_board(places.0), start_board(places.1));
    let mut scores: (usize, usize) = (0, 0);
    let mut roles: usize = 0;
    let mut dice = DETERMINISTIC_DICE.cycle();

    loop {
        let role: usize = dice.by_ref().take(3).sum();
        let place = places.0.by_ref().take(role).last().unwrap();
        scores.0 += place;
        roles += 3;
        if scores.0 >= 1000 {
            return Some(roles * scores.1);
        }

        let role: usize = dice.by_ref().take(3).sum();
        let place = places.1.by_ref().take(role).last().unwrap();
        scores.1 += place;
        roles += 3;
        if scores.1 >= 1000 {
            return Some(roles * scores.0);
        }
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

const BOARD: std::ops::RangeInclusive<usize> = 1..=10;
const DETERMINISTIC_DICE: std::ops::RangeInclusive<usize> = 1..=100;

fn parse_input(input: &str) -> (usize, usize) {
    input.split_once('\n').iter()
        .map(|(p1, p2)| {
            (p1.trim().split(' ').last().unwrap().parse().unwrap(), p2.trim().split(' ').last().unwrap().parse().unwrap())
        })
        .next().unwrap()
}

fn start_board(place : usize) -> Box<dyn Iterator<Item = usize>> {
    let mut board = BOARD.cycle();
    let _ = board.by_ref().take(place).last().unwrap();
    Box::new(board)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
