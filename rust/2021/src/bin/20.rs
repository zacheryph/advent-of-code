use std::cmp;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let (algo, image) = parse_input(input);
    let image = process_image(&algo, &image, 1);
    let image = process_image(&algo, &image, 2);

    Some(image.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (algo, mut image) = parse_input(input);

    for step in 0..50 {
        image = process_image(&algo, &image, step);
    }

    Some(image.len())
}

fn parse_input(input: &str) -> (HashSet<u16>, HashSet<(u32, u32)>) {
    let mut algorithm: HashSet<u16> = HashSet::new();
    let mut input_image: HashSet<(u32, u32)> = HashSet::new();

    let (raw_algo, raw_image) = input.split_once("\n\n").unwrap();

    raw_algo.chars().enumerate()
        .filter(|(_, c)| *c == '#')
        .for_each(|(idx, _)| { algorithm.insert(idx as u16); });

    raw_image.lines().enumerate()
        .for_each(|(line_no, line)| {
            line.chars().enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(idx, _)| { input_image.insert(((line_no + 100) as u32, (idx + 100) as u32)); });
        });

    (algorithm, input_image)
}


fn adjacent_coords(coord: &(u32, u32)) -> [(u32, u32); 9] {
    [
        (coord.0 - 1, coord.1 - 1), (coord.0 - 1, coord.1), (coord.0 - 1, coord.1 + 1),
        (coord.0,     coord.1 - 1), (coord.0,     coord.1), (coord.0,     coord.1 + 1),
        (coord.0 + 1, coord.1 - 1), (coord.0 + 1, coord.1), (coord.0 + 1, coord.1 + 1),
    ]
}
fn pixel_bits(coord: &(u32, u32), image: &HashSet<(u32, u32)>) -> u16 {
    let mut res = 0;

    for b in adjacent_coords(coord) {
        res <<= 1;
        if image.contains(&b) { res |= 1; }
    }

    res
}

fn process_image(algo: &HashSet<u16>, image: &HashSet<(u32 ,u32)>, step: u32) -> HashSet<(u32 ,u32)> {
    let mut result: HashSet<(u32 ,u32)> = HashSet::new();

    // this is absolutely hidious. since the surround
    // blinks on/off we just shrink the area we care
    // about by one for every step, giving us a giant
    // buffer between our possible change region.
    (1+step..300-step).for_each(|row| {
        (1+step..300-step).for_each(|col| {
            if algo.contains(&pixel_bits(&(row, col), &image)) {
                result.insert((row, col));
            }
        })
    });

    result
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
