use std::collections::HashMap;

fn main() {
    let map: Vec<Vec<char>> = std::fs::read_to_string("input/day10")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let s = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, c)| (*c == 'S').then_some((i as isize, j as isize)))
        })
        .unwrap();

    const ALL_DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let char_directions = {
        let mut map = HashMap::<char, [(isize, isize); 2]>::new();
        map.insert('|', [(1, 0), (-1, 0)]);
        map.insert('-', [(0, 1), (0, -1)]);
        map.insert('L', [(-1, 0), (0, 1)]);
        map.insert('J', [(-1, 0), (0, -1)]);
        map.insert('7', [(1, 0), (0, -1)]);
        map.insert('F', [(1, 0), (0, 1)]);
        map
    };

    let char_at = |pos: (isize, isize)| -> Option<char> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        };
        map.get(pos.0 as usize)?.get(pos.1 as usize).copied()
    };

    let mut pos = s;
    let mut prev = None::<(isize, isize)>;
    let mut loop_positions = vec![];
    while !loop_positions.contains(&pos) {
        loop_positions.push(pos);
        (prev, pos) = (Some(pos), {
            let c = char_at(pos).unwrap();
            if c == 'S' {
                ALL_DIRECTIONS
                    .into_iter()
                    .find_map(|dir| {
                        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                        let next_char = char_at(next_pos)?;
                        let connected = char_directions.get(&next_char)?;
                        connected.contains(&(-dir.0, -dir.1)).then_some(next_pos)
                    })
                    .unwrap()
            } else {
                let connected = char_directions.get(&c).unwrap();
                let prev = prev.unwrap();
                let dir = (pos.0 - prev.0, pos.1 - prev.1);
                let other_dir = connected
                    .iter()
                    .copied()
                    .find(|&d| d != (-dir.0, -dir.1))
                    .unwrap();
                (pos.0 + other_dir.0, pos.1 + other_dir.1)
            }
        });
    }

    let answer = loop_positions.len() / 2;
    println!("{answer}");
}
