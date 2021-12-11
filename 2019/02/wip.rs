const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");

fn parse_input() -> Vec<i64> {
    INPUT
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

fn process(program: &mut Vec<i64>) -> Result<i64, &'static str> {
    let mut pointer = 0;

    loop {
        // println!("LEN:{} PTR:{} CUR: {:?}", program.len(), pointer, Vec::from(&program[pointer..pointer+4]));
        match program[pointer..pointer + 4] {
            [99, ..] => return Ok(program[0]),
            [op, in1, in2, out] => {
                // println!("Process: {}({}, {}) => {}", op, in1, in2, out);
                program[out as usize] = opcode(op, (program[in1 as usize], program[in2 as usize]));
                pointer += 4;
            }
            [] => return Err("reached the end?"),
            [..] => {}
            // [_] | [_, _] | [_, _, _] => { return Err("something went wrong") },
        }
    }
}

fn stage_one() -> Result<i64, &'static str> {
    let mut program = parse_input();
    program[1] = 12;
    program[2] = 2;
    process(&mut program)
}

fn stage_two() -> i64 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = parse_input();
            program[1] = noun;
            program[2] = verb;
            match process(&mut program) {
                Ok(19690720) => {
                    println!("FOUND: noun:{} verb:{}", noun, verb);
                    return 100 * noun + verb;
                }
                _ => {}
            }
        }
    }
    0
}

fn main() {
    // Stage 1:
    println!("Stage 1: {:?}", stage_one());
    // Stage 2:
    println!("Stage 2: {:?}", stage_two());
}
