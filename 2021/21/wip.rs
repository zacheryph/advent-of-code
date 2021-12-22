const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");

const BOARD: std::ops::RangeInclusive<usize> = 1..=10;
const DETERMINISTIC_DICE: std::ops::RangeInclusive<usize> = 1..=100;

fn parse_input() -> (usize, usize) {
    INPUT.split_once("\n").iter()
        .map(|(p1, p2)| {
            (p1.trim().split(" ").last().unwrap().parse().unwrap(), p2.trim().split(" ").last().unwrap().parse().unwrap())
        })
        .next().unwrap()
}

fn start_board(place : usize) -> Box<dyn Iterator<Item = usize>> {
    let mut board = BOARD.cycle();
    let _ = board.by_ref().take(place).last().unwrap();
    Box::new(board)
}

fn stage_one() -> usize {
    let places = parse_input();
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
            return roles * scores.1;
        }

        let role: usize = dice.by_ref().take(3).sum();
        let place = places.1.by_ref().take(role).last().unwrap();
        scores.1 += place;
        roles += 3;
        if scores.1 >= 1000 {
            return roles * scores.0;
        }
    }
}

fn stage_two() -> i64 {
    0
}

fn main() {
    // Stage 1:
    println!("Stage 1: {}", stage_one());
    // Stage 2:
    println!("Stage 2: {}", stage_two());
}
