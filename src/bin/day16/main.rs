use std::collections::HashSet;

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");

    let input = if std::env::args().any(|arg| arg == "example") {
        include_str!("example")
    } else {
        include_str!("input")
    };

    #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
    enum Tile {
        Mirror,
        BackMirror,
        VerticalSplitter,
        HorizontalSplitter,
        Empty,
    }

    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '/' => Tile::Mirror,
                    '\\' => Tile::BackMirror,
                    '|' => Tile::VerticalSplitter,
                    '-' => Tile::HorizontalSplitter,
                    '.' => Tile::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        fn delta(&self) -> (i32, i32) {
            match self {
                Direction::Up => (-1, 0),
                Direction::Down => (1, 0),
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
            }
        }
    }

    #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
    struct Pos {
        row: i32,
        col: i32,
        dir: Direction,
    }

    impl Pos {
        fn go(self, dir: Direction) -> Self {
            let (delta_row, delta_col) = dir.delta();
            Self {
                row: self.row + delta_row,
                col: self.col + delta_col,
                dir,
            }
        }
    }

    struct Solver {
        map: Vec<Vec<Tile>>,
        visited: HashSet<Pos>,
    }
    impl Solver {
        fn new(map: Vec<Vec<Tile>>) -> Self {
            Self {
                map,
                visited: HashSet::new(),
            }
        }
        fn get_tile(&self, row: i32, col: i32) -> Option<Tile> {
            let tile = self
                .map
                .get(TryInto::<usize>::try_into(row).ok()?)?
                .get(TryInto::<usize>::try_into(col).ok()?)?;
            Some(*tile)
        }
        fn visit(&mut self, pos: Pos) {
            let Some(tile) = self.get_tile(pos.row, pos.col) else {
                return;
            };
            if !self.visited.insert(pos) {
                return;
            }
            match tile {
                Tile::Empty => self.visit(pos.go(pos.dir)),
                Tile::BackMirror => {
                    let new_dir = match pos.dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    self.visit(pos.go(new_dir));
                }
                Tile::Mirror => {
                    let new_dir = match pos.dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    self.visit(pos.go(new_dir));
                }
                Tile::HorizontalSplitter => match pos.dir {
                    Direction::Left | Direction::Right => {
                        self.visit(pos.go(pos.dir));
                    }
                    Direction::Up | Direction::Down => {
                        self.visit(pos.go(Direction::Left));
                        self.visit(pos.go(Direction::Right));
                    }
                },
                Tile::VerticalSplitter => match pos.dir {
                    Direction::Up | Direction::Down => {
                        self.visit(pos.go(pos.dir));
                    }
                    Direction::Left | Direction::Right => {
                        self.visit(pos.go(Direction::Up));
                        self.visit(pos.go(Direction::Down));
                    }
                },
            }
        }
        fn solve(mut self, pos: Pos) -> usize {
            self.visit(pos);
            self.visited
                .into_iter()
                .map(|pos| (pos.row, pos.col))
                .collect::<HashSet<_>>()
                .len()
        }
    }

    let answer = if part1 {
        Solver::new(map).solve(Pos {
            row: 0,
            col: 0,
            dir: Direction::Right,
        })
    } else {
        itertools::chain![
            (0..map.len()).map(|row| Pos {
                row: row as i32,
                col: 0,
                dir: Direction::Right,
            }),
            (0..map.len()).map(|row| Pos {
                row: row as i32,
                col: map.len() as i32 - 1,
                dir: Direction::Left,
            }),
            (0..map[0].len()).map(|col| Pos {
                row: 0,
                col: col as i32,
                dir: Direction::Down,
            }),
            (0..map[0].len()).map(|col| Pos {
                row: map.len() as i32 - 1,
                col: col as i32,
                dir: Direction::Up,
            })
        ]
        .map(|pos| Solver::new(map.clone()).solve(pos))
        .max()
        .unwrap()
    };

    println!("{answer}");
}
