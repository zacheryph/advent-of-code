use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<usize> {
    let mut graph = Graph::new(input);

    let mut paths: Vec<String> = Vec::new();
    let mut current: Vec<String> = Vec::from(["start".into()]);
    stage_one_crawl(&mut graph, &mut paths, &mut current);

    Some(paths.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut graph = Graph::new(input);
    let mut paths: Vec<String> = Vec::new();
    let mut current: Vec<String> = Vec::from(["start".into()]);

    stage_two_crawl(&mut graph, &mut paths, &mut current);
    Some(paths.len())
}

fn stage_one_crawl(graph: &mut Graph, paths: &mut Vec<String>, root: &mut Vec<String>) {
    let end = &root[root.len() - 1];
    let child_nodes = graph.map.get(end).unwrap().clone();
    for node in child_nodes {
        if node.eq(&node.to_lowercase()) && root.contains(&node) {
            continue;
        }

        if node.eq("end") {
            let full_path = format!("{},end", root.join(","));
            paths.push(full_path);
            continue;
        }

        root.push(node.clone());
        stage_one_crawl(graph, paths, root);
        root.pop();
    }
}

fn stage_two_crawl(graph: &mut Graph, paths: &mut Vec<String>, root: &mut Vec<String>) {
    let end = &root[root.len() - 1];
    let child_nodes = graph.map.get(end).unwrap().clone();
    for node in child_nodes {
        if "start".eq(&node) {
            continue;
        }

        if node.eq("end") {
            let full_path = format!("{},end", root.join(","));
            paths.push(full_path);
            continue;
        }

        if node.eq(&node.to_lowercase()) {
            let (uniq, lower) =
                root.iter()
                    .fold((HashSet::new(), 0), |(mut uniq, mut lower), node| {
                        if node.eq(&node.to_lowercase()) {
                            uniq.insert(node);
                            lower += 1;
                        }
                        (uniq, lower)
                    });

            if uniq.contains(&node) && uniq.len() != lower {
                continue;
            }
        }

        root.push(node.clone());
        stage_two_crawl(graph, paths, root);
        root.pop();
    }
}

struct Graph {
    map: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut res = Self {
            map: HashMap::new(),
        };

        input.lines().for_each(|l| {
            let (a, b) = l.split_once("-").unwrap();
            res.insert_link([a, b]);
            res.insert_link([b, a])
        });

        res
    }

    fn insert_link(&mut self, link: [&str; 2]) {
        match self.map.get_mut(link[0]) {
            Some(dest) => dest.push(link[1].into()),
            None => {
                self.map.insert(link[0].into(), Vec::from([link[1].into()]));
            }
        };
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
