use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i64> {
    Some(calculate_stage(input, 10))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(calculate_stage(input, 40))
}

type ExpandMap = HashMap<(char, char), [(char, char); 2]>;
type CountMap = HashMap<(char, char), HashMap<char, i64>>;

fn parse_input(input: &str) -> (Vec<char>, ExpandMap, CountMap) {
    let (template, mappings) = input.split_once("\n\n").unwrap();

    let mut expand_map = ExpandMap::new();
    let mut base_count = CountMap::new();

    mappings.lines().for_each(|l| {
        let (k, v) = l.split_once(" -> ").unwrap();
        let k: Vec<char> = k.chars().collect();
        let mid = v.chars().nth(0).unwrap();

        expand_map.insert((k[0], k[1]), [(k[0], mid), (mid, k[1])]);
        base_count.insert((k[0], k[1]), HashMap::from([(mid, 1)]));
    });

    (template.chars().collect(), expand_map, base_count)
}

fn depth_map(depth: i64, base: &CountMap, mappings: &ExpandMap) -> CountMap {
    (2..=depth).into_iter().fold(base.clone(), |acc, _| {
        let mut count = CountMap::new();

        for (k, _) in &acc {
            let split = mappings.get(&k).unwrap();
            let left = acc.get(&split[0]).unwrap();
            let right = acc.get(&split[1]).unwrap();

            count.insert(*k, left.iter().chain(right.iter()).fold(HashMap::new(), |mut acc, (ch, cnt)| {
                let ptr = acc.entry(*ch).or_insert(0);
                *ptr += cnt;
                acc
            }));

            let update = count.get_mut(&k).unwrap();
            let ptr = update.entry(split[0].1).or_insert(0);
            *ptr += 1
        }

        count
    })
}

fn reduce_template(template: &Vec<char>, map: &CountMap) -> HashMap<char, i64> {
    let mut totals: HashMap<char, i64> = HashMap::new();

    template.iter().for_each(|c| {
        let ptr = totals.entry(*c).or_insert(0);
        *ptr += 1;
    });

    template.as_slice().windows(2)
        .map(|chars| map.get(&(chars[0], chars[1])).unwrap())
        .fold(totals, |mut acc, charset| {
            charset.iter().for_each(|(ch, cnt)| {
                let ptr = acc.entry(*ch).or_insert(0);
                *ptr += cnt;
            });
            acc
    })
}

fn high_low(counts: &HashMap<char, i64>) -> (i64, i64) {
    let high = counts.iter().fold(0, |acc, (_, v)| if *v > acc { *v } else { acc });
    let low = counts.iter().fold(high, |acc, (_, v)| if *v < acc { *v } else { acc });
    (high, low)
}

fn calculate_stage(input: &str, depth: i64) -> i64 {
    let (template, mapping, base) = parse_input(input);

    let counts = depth_map(depth, &base, &mapping);
    let combined = reduce_template(&template, &counts);
    let (high, low) = high_low(&combined);

    high - low
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
