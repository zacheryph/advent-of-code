use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");

type ExpandMap = HashMap<(char, char), [(char, char); 2]>;
type CountMap = HashMap<(char, char), HashMap<char, i64>>;

fn parse_input() -> (Vec<char>, ExpandMap, CountMap) {
    let (template, mappings) = INPUT.split_once("\n\n").unwrap();

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

fn calculate_stage(depth: i64) -> i64 {
    let (template, mapping, base) = parse_input();

    let counts = depth_map(depth, &base, &mapping);
    let combined = reduce_template(&template, &counts);
    let (high, low) = high_low(&combined);

    high - low
}

fn main() {
    // Stage 1:
    println!("Stage 1: {}", calculate_stage(10));
    // Stage 2:
    println!("Stage 2: {}", calculate_stage(40));
}
