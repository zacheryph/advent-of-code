pub fn part_one(input: &str) -> Option<i64> {
    let n = input
        .lines()
        .filter_map(|l| parse_line(l).err())
        .fold(0, |acc, c| acc + char_points(c));

    Some(n)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut scores: Vec<i64> = input
        .lines()
        .filter_map(|l| parse_line(l).ok())
        .map(|stack| completion_score(stack))
        .collect();

    scores.sort();
    Some(scores[(scores.len() - 1) / 2])
}

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

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
