use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");
// const INPUT: &str = include_str!("test.txt");
// const INPUT: &str = include_str!("test1.txt");
// const INPUT: &str = include_str!("test2.txt");

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

fn stage_one() -> i64 {
    let mut graph = Graph::new(INPUT);
    // println!("MAP: {:?}", graph.map);

    let mut paths: Vec<String> = Vec::new();
    let mut current: Vec<String> = Vec::from(["start".into()]);
    stage_one_crawl(&mut graph, &mut paths, &mut current);

    // println!("PATHS: {:?}", paths);
    paths.len() as i64
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

fn stage_two() -> i64 {
    let mut graph = Graph::new(INPUT);
    let mut paths: Vec<String> = Vec::new();
    let mut current: Vec<String> = Vec::from(["start".into()]);

    stage_two_crawl(&mut graph, &mut paths, &mut current);
    paths.len() as i64
}

fn main() {
    // Stage 1:
    println!("Stage 1: {}", stage_one());
    // Stage 2:
    println!("Stage 2: {}", stage_two());
}
