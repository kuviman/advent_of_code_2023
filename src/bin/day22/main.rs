use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    ops::Deref,
};

use itertools::Itertools;

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");
    let example = std::env::args().any(|arg| arg == "example");

    let input = if example {
        include_str!("example")
    } else {
        include_str!("input")
    };

    #[allow(non_camel_case_types)]
    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct vec3 {
        x: i32,
        y: i32,
        z: i32,
    }

    impl Deref for vec3 {
        type Target = [i32; 3];
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute(self) }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Brick {
        ends: [vec3; 2],
    }

    impl Brick {
        fn xy(&self) -> impl Iterator<Item = [i32; 2]> {
            let x_range = self.ends[0].x..=self.ends[1].x;
            let y_range = self.ends[0].y..=self.ends[1].y;
            x_range.flat_map(move |x| y_range.clone().map(move |y| [x, y]))
        }
        fn size(&self) -> i32 {
            (0..3)
                .map(|c| self.ends[1][c] - self.ends[0][c] + 1)
                .max()
                .unwrap()
        }
    }

    let bricks: Vec<Brick> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('~').unwrap();
            let ends = [a, b].map(|end| {
                let coords: Vec<i32> = end.split(',').map(|x| x.parse().unwrap()).collect();
                let [x, y, z]: [i32; 3] = coords.try_into().unwrap();
                vec3 { x, y, z }
            });

            for coord in 0..3 {
                assert!(ends[0][coord] <= ends[1][coord]);
            }

            Brick { ends }
        })
        .collect();

    dbg!(bricks.len());
    dbg!(bricks.iter().map(Brick::size).max().unwrap());

    let mut fallen = Vec::<Brick>::with_capacity(bricks.len());

    let mut bricks = bricks;
    bricks.sort_by_key(|brick| brick.ends[0].z);

    let mut height_map = HashMap::<[i32; 2], usize>::new();

    let mut supported_by = Vec::<HashSet<usize>>::new();

    for mut brick in bricks {
        let this_idx = fallen.len();

        let bricks_below: HashSet<usize> = brick
            .xy()
            .flat_map(|xy| height_map.get(&xy).copied())
            .collect();

        let support_z = bricks_below
            .iter()
            .map(|&idx| fallen[idx].ends[1].z)
            .max()
            .unwrap_or(0);

        supported_by.push(
            bricks_below
                .iter()
                .copied()
                .filter(|&idx| fallen[idx].ends[1].z == support_z)
                .collect(),
        );

        let fall_to_z = support_z + 1;
        let fall_distance = brick.ends[0].z - fall_to_z;
        for end in &mut brick.ends {
            end.z -= fall_distance;
        }

        for xy in brick.xy() {
            height_map.insert(xy, this_idx);
        }

        fallen.push(brick);
    }

    let mut supports: Vec<HashSet<usize>> = vec![HashSet::new(); fallen.len()];
    #[allow(clippy::needless_range_loop)]
    for this_idx in 0..fallen.len() {
        for below in supported_by[this_idx].iter().copied() {
            supports[below].insert(this_idx);
        }
    }

    if part1 {
        let mut answer = 0;
        #[allow(clippy::needless_range_loop)]
        for this_idx in 0..fallen.len() {
            let can_disintegrate = !supports[this_idx]
                .iter()
                .copied()
                .any(|supported| supported_by[supported].len() == 1);
            if can_disintegrate {
                answer += 1;
            }
        }

        dbg!(answer);
    } else {
        // part 2
        let mut answer = 0;

        #[allow(clippy::needless_range_loop)]
        for start in 0..fallen.len() {
            let mut num_of_supported_by: Vec<usize> =
                supported_by.iter().map(|s| s.len()).collect();
            let mut q = vec![start];
            let mut fallen = 0;
            while let Some(disintegrated) = q.pop() {
                fallen += 1;
                for &above in &supports[disintegrated] {
                    num_of_supported_by[above] -= 1;
                    if num_of_supported_by[above] == 0 {
                        q.push(above);
                    }
                }
            }
            answer += fallen - 1;
        }

        dbg!(answer);
    }
}
