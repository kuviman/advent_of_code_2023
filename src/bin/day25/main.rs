use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use itertools::Itertools;
use rand::{thread_rng, Rng};

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");
    let example = std::env::args().any(|arg| arg == "example");

    let input = if example {
        include_str!("example")
    } else {
        include_str!("input")
    };

    type Node = Arc<str>;
    let g: HashMap<Node, Vec<Node>> = input
        .lines()
        .map(|line| {
            let (name, connections) = line.split_once(": ").unwrap();
            let name = name.into();
            let connections = connections.split_whitespace().map(|s| s.into()).collect();
            (name, connections)
        })
        .collect();

    println!("vertices = {}", g.len());
    println!(
        "edges = {}",
        g.values().map(|edges| edges.len()).sum::<usize>(),
    );

    let g: Vec<[Arc<str>; 2]> = g
        .into_iter()
        .flat_map(|(v, edges)| edges.into_iter().map(move |u| [v.clone(), u]))
        .collect();

    struct DSU<N: std::hash::Hash + Clone + Eq> {
        parents: HashMap<N, N>,
    }

    impl<N: std::hash::Hash + Clone + Eq> DSU<N> {
        fn new() -> Self {
            Self {
                parents: HashMap::new(),
            }
        }
        fn root(&mut self, v: N) -> N {
            let root = match self.parents.get(&v).cloned() {
                Some(parent) => self.root(parent),
                None => v.clone(),
            };
            if root != v {
                self.parents.insert(v, root.clone());
            }
            root
        }
        fn merge(&mut self, a: N, b: N) -> bool {
            let (a, b) = if thread_rng().gen() { (a, b) } else { (b, a) };
            let a = self.root(a);
            let b = self.root(b);
            if a != b {
                self.parents.insert(a, b);
                true
            } else {
                false
            }
        }
    }

    fn karger<N: std::hash::Hash + Clone + Eq + std::fmt::Debug>(g: &[[N; 2]]) -> [usize; 2] {
        loop {
            // dbg!(g);
            let mut edges = g.to_vec();
            let all_nodes = g.iter().flatten().cloned().collect::<HashSet<N>>();
            let mut vertices = all_nodes.len();
            let mut dsu = DSU::new();
            while vertices > 2 {
                let [a, b] = edges.swap_remove(thread_rng().gen_range(0..edges.len()));
                if dsu.merge(a, b) {
                    vertices -= 1;
                }
            }
            let edges_left = edges
                .iter()
                .filter(|[a, b]| dsu.root(a.clone()) != dsu.root(b.clone()))
                .count();

            if edges_left == 3 {
                let mut components = HashMap::<N, usize>::new();
                for v in all_nodes {
                    // println!("{v:?} -> {:?}", dsu.root(v.clone()));
                    *components.entry(dsu.root(v)).or_default() += 1;
                }
                dbg!(&components);
                return components.into_values().collect_vec().try_into().unwrap();
            }
        }
    }

    let [a, b] = dbg!(karger(&g));
    let answer = a * b;
    dbg!(answer);
}
