use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::Range,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct vec2(i64, i64);

impl std::ops::Add for vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<i64> for vec2 {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");

    let input = if std::env::args().any(|arg| arg == "example") {
        include_str!("example")
    } else {
        include_str!("input")
    };

    let commands: Vec<(vec2, i64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let dir = match parts.next().unwrap() {
                "L" => vec2(-1, 0),
                "R" => vec2(1, 0),
                "D" => vec2(0, -1),
                "U" => vec2(0, 1),
                _ => unreachable!(),
            };
            let num: i64 = parts.next().unwrap().parse().unwrap();
            let color = parts
                .next()
                .unwrap()
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(')')
                .unwrap();

            if part1 {
                (dir, num)
            } else {
                (
                    match &color[5..] {
                        "0" => vec2(1, 0),
                        "1" => vec2(0, -1),
                        "2" => vec2(-1, 0),
                        "3" => vec2(0, 1),
                        _ => unreachable!(),
                    },
                    i64::from_str_radix(&color[..5], 16).unwrap(),
                )
            }
        })
        .collect();

    let mut area = 0;
    let mut boundary = 0;
    let mut pos = vec2(0, 0);
    for &(dir, num) in &commands {
        area += dir.0 * pos.1 * num;
        pos = pos + dir * num;
        boundary += num;
    }
    let area = area.abs();
    assert!(boundary % 2 == 0);
    let inside = area + 1 - boundary / 2;
    let answer = boundary + inside;
    dbg!(boundary);
    dbg!(inside);
    dbg!(area);
    println!("{answer}");
}
