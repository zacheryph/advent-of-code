pub fn part_one(input: &str) -> Option<i64> {
    let mut program = parse_input(input);
    program[1] = 12;
    program[2] = 2;

    process(&mut program)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn opcode(code: i64, values: (i64, i64)) -> i64 {
    match code {
        1 => values.0 + values.1,
        2 => values.0 * values.1,
        _ => 0,
    }
}

fn process(program: &mut Vec<i64>) -> Option<i64> {
    let mut pointer = 0;

    loop {
        // println!("LEN:{} PTR:{} CUR: {:?}", program.len(), pointer, Vec::from(&program[pointer..pointer+4]));
        match program[pointer..pointer + 4] {
            [99, ..] => return Some(program[0]),
            [op, in1, in2, out] => {
                // println!("Process: {}({}, {}) => {}", op, in1, in2, out);
                program[out as usize] = opcode(op, (program[in1 as usize], program[in2 as usize]));
                pointer += 4;
            }
            [] => {
                println!("Error: reached the end?");
                return None
            }
            [..] => {}
            // [_] | [_, _] | [_, _, _] => { return Err("something went wrong") },
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
