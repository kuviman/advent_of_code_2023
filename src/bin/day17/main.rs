use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");

    let input = if std::env::args().any(|arg| arg == "example") {
        include_str!("example")
    } else {
        include_str!("input")
    };

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let map_size = vec2(map.len() as i32, map[0].len() as i32);

    let map = |pos: vec2| -> Option<u32> {
        Some(
            *map.get(TryInto::<usize>::try_into(pos.0).ok()?)?
                .get(TryInto::<usize>::try_into(pos.1).ok()?)?,
        )
    };

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct vec2(i32, i32);

    impl std::ops::Add for vec2 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Self(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Direction(i32);

    impl Direction {
        const RIGHT: Self = Self(0);
        const DOWN: Self = Self(1);
        const LEFT: Self = Self(2);
        const UP: Self = Self(3);
    }

    impl Direction {
        fn to_vec(self) -> vec2 {
            match self.0 {
                0 => vec2(1, 0),
                1 => vec2(0, 1),
                2 => vec2(-1, 0),
                3 => vec2(0, -1),
                _ => unreachable!(),
            }
        }
    }

    impl std::ops::Add<i32> for Direction {
        type Output = Self;

        fn add(self, rhs: i32) -> Self {
            Self((self.0 + rhs).rem_euclid(4))
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Pos {
        pos: vec2,
        moves_without_rotation: usize,
        dir: Direction,
    }

    #[derive(PartialEq, Eq, Debug)]
    struct QueuedPos {
        pos: Pos,
        dist: u32,
    }

    impl PartialOrd for QueuedPos {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for QueuedPos {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.dist.cmp(&other.dist).reverse()
        }
    }

    let mut q = BinaryHeap::new();
    let mut distances = HashMap::new();

    let starts = [Direction::DOWN, Direction::RIGHT]
        .into_iter()
        .map(|dir| Pos {
            pos: vec2(0, 0),
            moves_without_rotation: 0,
            dir,
        });
    for pos in starts {
        distances.insert(pos, 0);
        q.push(QueuedPos { pos, dist: 0 });
    }
    println!("{map_size:?}");
    while let Some(queued) = q.pop() {
        if distances[&queued.pos] != queued.dist {
            continue;
        }
        if queued.pos.pos == map_size + vec2(-1, -1) {
            println!("{}", queued.dist);
            return;
        }
        let pos = queued.pos;

        let (can_continue_forward, can_rotate) = if part1 {
            (pos.moves_without_rotation < 3, true)
        } else {
            (
                pos.moves_without_rotation < 10,
                pos.moves_without_rotation >= 4,
            )
        };

        let next_moves = itertools::chain![
            can_continue_forward.then(|| {
                Pos {
                    pos: pos.pos + pos.dir.to_vec(),
                    moves_without_rotation: pos.moves_without_rotation + 1,
                    dir: pos.dir,
                }
            }),
            [-1, 1].into_iter().filter(|_| can_rotate).map(|rot| {
                let new_dir = pos.dir + rot;
                Pos {
                    pos: pos.pos + new_dir.to_vec(),
                    moves_without_rotation: 1,
                    dir: new_dir,
                }
            }),
        ];

        for next_pos in next_moves {
            if let Some(tile) = map(next_pos.pos) {
                let new_dist = queued.dist + tile;
                if distances
                    .get(&next_pos)
                    .map_or(true, |&prev_dist| new_dist < prev_dist)
                {
                    distances.insert(next_pos, new_dist);
                    q.push(QueuedPos {
                        pos: next_pos,
                        dist: new_dist,
                    });
                }
            }
        }
    }
}
