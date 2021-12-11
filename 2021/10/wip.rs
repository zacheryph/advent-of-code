const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");

fn char_points(code: char) -> i64 {
    match code {
        // stage 1
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        // stage 2
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn valid_chunk(chunk: (char, char)) -> bool {
    match chunk {
        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => true,
        _ => false,
    }
}

fn parse_line(line: &str) -> Result<Vec<char>, char> {
    let mut stack: Vec<char> = Vec::new();

    let res = line.chars().find(|c| match c {
        ')' | ']' | '}' | '>' => match stack.pop() {
            None => true,
            Some(open) => {
                if valid_chunk((open, *c)) {
                    false
                } else {
                    true
                }
            }
        },
        c => {
            stack.push(*c);
            false
        }
    });

    match res {
        Some(c) => Err(c),
        _ => Ok(stack),
    }
}

fn completion_score(stack: Vec<char>) -> i64 {
    stack
        .iter()
        .rev()
        .fold(0, |acc, c| (acc * 5) + char_points(*c))
}

fn stage_one() -> i64 {
    INPUT
        .lines()
        .filter_map(|l| parse_line(l).err())
        .fold(0, |acc, c| acc + char_points(c))
}

fn stage_two() -> i64 {
    let mut scores: Vec<i64> = INPUT
        .lines()
        .filter_map(|l| parse_line(l).ok())
        .map(|stack| completion_score(stack))
        .collect();

    scores.sort();
    scores[(scores.len() - 1) / 2]
}

fn main() {
    // Stage 1: 345441
    println!("Stage 1: {}", stage_one());
    // Stage 2: 3235371166
    println!("Stage 2: {}", stage_two());
}
