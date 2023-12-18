use std::collections::HashMap;

fn main() {
    let part2 = std::env::args().any(|arg| arg == "part2");

    let input = if std::env::args().any(|arg| arg == "example") {
        include_str!("example")
    } else {
        include_str!("input")
    };

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
    enum Tile {
        RoundRock,
        BoxRock,
        Empty,
    }
    let input: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Tile::RoundRock,
                    '#' => Tile::BoxRock,
                    '.' => Tile::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    fn go_up(map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        let mut map = map;
        for col in 0..map[0].len() {
            let mut last_place = 0;
            for row in 0..map.len() {
                match map[row][col] {
                    Tile::Empty => {}
                    Tile::BoxRock => {
                        last_place = row + 1;
                    }
                    Tile::RoundRock => {
                        map[row][col] = Tile::Empty;
                        map[last_place][col] = Tile::RoundRock;
                        last_place += 1;
                    }
                }
            }
        }
        map
    }

    fn rotate_clockwise(map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        (0..map[0].len())
            .map(|col| (0..map.len()).rev().map(|row| map[row][col]).collect())
            .collect()
    }

    fn cycle(map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        let mut map = map;
        for _ in 0..4 {
            map = go_up(map);
            map = rotate_clockwise(map);
        }
        map
    }

    fn calculate_load(map: &Vec<Vec<Tile>>) -> u64 {
        let mut answer = 0;
        for (row_index, row) in map.iter().enumerate() {
            for col in row {
                if let Tile::RoundRock = col {
                    answer += map.len() - row_index;
                }
            }
        }
        answer as u64
    }

    fn print_debug(map: &Vec<Vec<Tile>>) {
        for line in map {
            let line: String = line
                .iter()
                .map(|tile| match tile {
                    Tile::RoundRock => 'O',
                    Tile::BoxRock => '#',
                    Tile::Empty => '.',
                })
                .collect();
            println!("{line}");
        }
        println!();
    }

    if part2 {
        let mut map = input;

        let mut visited = HashMap::new();

        const CYCLES: usize = 1_000_000_000;
        for cycle_index in 0..CYCLES {
            // print_debug(&map);
            // println!("={}\n", calculate_load(&map));

            if let Some(prev_index) = visited.insert(map.clone(), cycle_index) {
                let cycle_len = cycle_index - prev_index;
                println!("loop = {cycle_len}");
                let remaining_cycles = (CYCLES - cycle_index) % cycle_len;
                for _ in 0..remaining_cycles {
                    map = cycle(map);
                }
                break;
            }
            map = cycle(map);
        }

        let answer = calculate_load(&map);
        println!("{answer}");
    } else {
        let answer = calculate_load(&go_up(input));
        println!("{answer}");
    }
}
