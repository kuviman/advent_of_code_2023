use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");
    let example = std::env::args().any(|arg| arg == "example");

    let input = if example {
        include_str!("example")
    } else {
        include_str!("input")
    };

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Tile {
        Garden,
        Rock,
    }

    let mut start = None;
    let map: Vec<Vec<Tile>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Tile::Garden,
                    '#' => Tile::Rock,
                    'S' => {
                        assert!(start.replace((row as i32, col as i32)).is_none());
                        Tile::Garden
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // make sure map size is even
    let map: Vec<Vec<Tile>> = std::iter::repeat(&map)
        .take(2)
        .flatten()
        .map(|row| std::iter::repeat(row).take(2).flatten().copied().collect())
        .collect();

    let start = start.unwrap();

    let map_size = (map.len() as i32, map[0].len() as i32);

    let get_tile = |x: i32, y: i32| -> Tile {
        map[x.rem_euclid(map_size.0 as i32) as usize][y.rem_euclid(map_size.1 as i32) as usize]
    };

    let max_distance = if part1 {
        if example {
            6
        } else {
            64
        }
    } else {
        // part 2
        if example {
            5000
        } else {
            26501365
        }
    };

    let max_honest_distance = (map_size.0.max(map_size.1)) * 5;

    let mut honest_distance = HashMap::<(i32, i32), u64>::from_iter([(start, 0)]);
    let mut q = VecDeque::from_iter([start]);
    while let Some((x, y)) = q.pop_front() {
        let current_distance = honest_distance[&(x, y)];
        if x.abs().max(y.abs()) > max_honest_distance {
            continue;
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;

            if part1 && (nx < 0 || ny < 0 || nx >= map_size.0 as i32 || ny >= map_size.1 as i32) {
                continue;
            }
            let tile = get_tile(nx, ny);
            if tile == Tile::Rock {
                continue;
            }

            if let std::collections::hash_map::Entry::Vacant(entry) =
                honest_distance.entry((nx, ny))
            {
                entry.insert(current_distance + 1);
                q.push_back((nx, ny));
            }
        }
    }

    let reachable_origin: HashSet<(i32, i32)> = honest_distance
        .keys()
        .copied()
        .filter(|&(x, y)| 0 <= x && x < map_size.0 && 0 <= y && y < map_size.1)
        .collect();

    let distance = {
        let honest_distance = &honest_distance;
        let reachable_origin = &reachable_origin;
        let get_distance = |cx: i32, cy: i32, ix: i32, iy: i32| -> Option<u64> {
            honest_distance
                .get(&(cx * map_size.0 + ix, cy * map_size.1 + iy))
                .copied()
        };
        let clamp = |c: i32, extra: i32| -> i32 {
            let max = 3;
            c.clamp(-max - extra, max + extra)
        };
        move |x: i32, y: i32| -> Option<u64> {
            let cx = x.div_euclid(map_size.0);
            let cy = y.div_euclid(map_size.1);
            let ix = x.rem_euclid(map_size.0);
            let iy = y.rem_euclid(map_size.1);

            if !reachable_origin.contains(&(ix, iy)) {
                return None;
            }

            let step_x = get_distance(clamp(cx, 1), clamp(cy, 0), ix, iy).unwrap()
                - get_distance(clamp(cx, 0), clamp(cy, 0), ix, iy).unwrap();

            let step_y = get_distance(clamp(cx, 0), clamp(cy, 1), ix, iy).unwrap()
                - get_distance(clamp(cx, 0), clamp(cy, 0), ix, iy).unwrap();

            let result = get_distance(clamp(cx, 0), clamp(cy, 0), ix, iy).unwrap()
                + step_x * (cx - clamp(cx, 0)).unsigned_abs() as u64
                + step_y * (cy - clamp(cy, 0)).unsigned_abs() as u64;

            if let Some(&honest) = honest_distance.get(&(x, y)) {
                assert_eq!(honest, result, "{x}, {y}")
            }

            Some(result)
        }
    };

    // for x in 0..100 {
    //     if x % map_size.0 == 0 {
    //         println!("===");
    //     }
    //     if let Some(distance) = honest_distance.get(&(x, 5)) {
    //         println!("{distance}");
    //     } else {
    //         println!("rock");
    //     }
    // }

    // for y in (-60..0).rev() {
    //     if y % map_size.1 == 0 {
    //         println!("===");
    //     }
    //     let x = 3;
    //     print!("{x}, {y}: ");
    //     if let Some(distance) = honest_distance.get(&(x, y)) {
    //         println!("{distance}");
    //     } else {
    //         println!("rock");
    //     }
    // }

    for &(x, y) in honest_distance.keys() {
        distance(x, y);
    }

    let distance = |x, y| distance(x, y).filter(|&distance| distance <= max_distance);

    let reachable = |cx: i32, cy: i32| -> bool {
        for x in [0, map_size.0 - 1] {
            for y in [0, map_size.1 - 1] {
                let ax = cx * map_size.0 as i32 + x;
                let ay = cy * map_size.1 as i32 + y;
                if distance(ax, ay).is_some() {
                    return true;
                }
            }
        }
        false
    };
    let fully_reachable = |cx: i32, cy: i32| -> bool {
        for x in [0, map_size.0 - 1] {
            for y in [0, map_size.1 - 1] {
                let ax = cx * map_size.0 as i32 + x;
                let ay = cy * map_size.1 as i32 + y;
                if reachable_origin.contains(&(x, y)) && distance(ax, ay).is_none() {
                    return false;
                }
            }
        }
        true
    };

    let garden_tiles: [u64; 2] = std::array::from_fn(|rem| {
        let mut sum = 0;
        for x in 0..map_size.0 {
            for y in 0..map_size.1 {
                if reachable_origin.contains(&(x, y)) && distance(x, y).unwrap() % 2 == rem as _ {
                    sum += 1;
                }
            }
        }
        sum
    });

    let calculate_partially_reachable = |cx: i32, cy: i32| -> u64 {
        let mut sum = 0;
        for ix in 0..map_size.0 {
            for iy in 0..map_size.1 {
                if let Some(distance) = distance(cx * map_size.0 + ix, cy * map_size.1 + iy) {
                    if distance % 2 == max_distance % 2 {
                        sum += 1;
                    }
                }
            }
        }
        sum
    };

    dbg!(map_size);

    let mut answer: u64 = 0;

    let min_x = (0..).map(|cx| -cx).find(|&cx| !reachable(cx, 0)).unwrap() + 1;

    let mut min_y = 0;
    let mut max_y = 0;

    let mut prev_partial = None;

    for cx in min_x.. {
        if !reachable(cx, 0) {
            break;
        }

        // dbg!(cx);

        let mut ops = 0;

        while reachable(cx, min_y) {
            ops += 1;
            min_y -= 1;
        }
        while reachable(cx, max_y) {
            ops += 1;
            max_y += 1;
        }

        while !reachable(cx, min_y) {
            ops += 1;
            min_y += 1;
        }
        while !reachable(cx, max_y) {
            ops += 1;
            max_y -= 1;
        }

        // dbg!(cx);

        // dbg!(min_y, max_y);

        let mut ops = 0;
        let mut partial = 0;

        let calculate_partial = cx.abs() < 10 || cx.abs().abs_diff(min_x.abs()) < 10;

        for cy in min_y..=-1 {
            ops += 1;
            if fully_reachable(cx, cy) {
                assert!(fully_reachable(cx, -1));
                answer += (-1 - cy + 1) as u64 * garden_tiles[max_distance as usize % 2];
                break;
            }
            if calculate_partial {
                partial += calculate_partially_reachable(cx, cy);
            }
        }
        for cy in (0..=max_y).rev() {
            ops += 1;
            if fully_reachable(cx, cy) {
                assert!(fully_reachable(cx, 0));
                answer += (cy + 1) as u64 * garden_tiles[max_distance as usize % 2];
                break;
            }
            if calculate_partial {
                partial += calculate_partially_reachable(cx, cy);
            }
        }
        if !calculate_partial {
            partial = prev_partial.unwrap();
        }
        prev_partial = Some(partial);
        answer += partial;
        dbg!(cx, ops, partial);
    }

    dbg!(answer);
}
