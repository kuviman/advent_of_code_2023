use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    ops::Deref,
    sync::Arc,
};

use itertools::Itertools;

type Pos = [i32; 2];

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Tile {
    Path,
    Forest,
    Slope(Pos),
}

#[derive(Debug)]
struct Edge {
    to: Pos,
    len: usize,
}

struct Solver {
    map: Vec<Vec<Tile>>,
    g: HashMap<Pos, Arc<[Edge]>>,
    visited: HashSet<Pos>,
    current_len: usize,
    answer: Option<usize>,
    entrance: Pos,
    exit: Pos,
    part1: bool,
    ways: usize,
}

impl Solver {
    fn brute(&mut self, pos: Pos, prev: Option<Pos>) {
        if pos[0] == self.map.len() as i32 - 1 {
            self.answer = self.answer.max(Some(self.current_len));
            self.ways += 1;
            return;
        }
        // let distance_to_exit = (pos[0] - self.exit[0]).abs() + (pos[1] - self.exit[1]).abs();
        if !self.visited.contains(&pos) {
            self.visited.insert(pos);
            let edges = self.g[&pos].clone();
            for edge in &*edges {
                self.current_len += edge.len;
                self.brute(edge.to, Some(pos));
                self.current_len -= edge.len;
            }
            self.visited.remove(&pos);
        }
    }
    fn solve(mut self) -> usize {
        self.brute(self.entrance, None);
        dbg!(self.ways);
        self.answer.unwrap()
    }

    fn new(map: Vec<Vec<Tile>>, part1: bool) -> Self {
        let find = |row: usize| {
            [
                row as i32,
                map[row]
                    .iter()
                    .position(|tile| *tile == Tile::Path)
                    .unwrap() as i32,
            ]
        };
        let entrance = find(0);
        let exit = find(map.len() - 1);

        let mut important = HashSet::<Pos>::new();
        let get_tile = |pos: Pos| -> Option<Tile> {
            let [row, col]: [Option<usize>; 2] = pos.map(|x| x.try_into().ok());
            let [row, col] = [row?, col?];
            map.get(row)?.get(col).copied()
        };
        let neighbors = |pos: Pos| -> Vec<Pos> {
            let all_directions = vec![[-1, 0], [1, 0], [0, -1], [0, 1]];
            let can_go = if part1 {
                match get_tile(pos).unwrap() {
                    Tile::Path => all_directions,
                    Tile::Forest => unreachable!(),
                    Tile::Slope(dir) => vec![dir],
                }
            } else {
                all_directions
            };
            can_go
                .into_iter()
                .filter_map(|[dx, dy]| {
                    let [x, y] = pos;
                    let new_pos = [x + dx, y + dy];
                    match get_tile(new_pos) {
                        Some(Tile::Forest) | None => None,
                        _ => Some(new_pos),
                    }
                })
                .collect()
        };
        #[allow(clippy::needless_range_loop)]
        for x in 0..map.len() {
            for y in 0..map[x].len() {
                if map[x][y] == Tile::Forest {
                    continue;
                }
                let pos = [x as i32, y as i32];
                if pos == entrance || pos == exit || neighbors(pos).len() > 2 {
                    important.insert(pos);
                }
            }
        }

        let mut g = HashMap::new();
        for &v in &important {
            let mut distance = HashMap::<Pos, usize>::from_iter([(v, 0)]);
            let mut q = VecDeque::from_iter([v]);
            let mut edges = Vec::new();
            while let Some(pos) = q.pop_front() {
                let current_distance = distance[&pos];
                if pos != v && important.contains(&pos) {
                    edges.push(Edge {
                        len: current_distance,
                        to: pos,
                    });
                    continue;
                }
                for next in neighbors(pos) {
                    if let std::collections::hash_map::Entry::Vacant(e) = distance.entry(next) {
                        e.insert(current_distance + 1);
                        q.push_back(next);
                    }
                }
            }
            g.insert(v, edges.into());
        }

        dbg!(g.len());

        Self {
            ways: 0,
            map,
            g,
            visited: Default::default(),
            part1,
            entrance,
            exit,
            answer: None,
            current_len: 0,
        }
    }
}

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");
    let example = std::env::args().any(|arg| arg == "example");

    let input = if example {
        include_str!("example")
    } else {
        include_str!("input")
    };

    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '<' => Tile::Slope([0, -1]),
                    '>' => Tile::Slope([0, 1]),
                    'v' => Tile::Slope([1, 0]),
                    '^' => Tile::Slope([-1, 0]),
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let answer = Solver::new(map, part1).solve();
    dbg!(answer);
}
