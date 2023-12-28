use std::ops::{Range, RangeInclusive};

use batbox_la::*;
use batbox_num::*;
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng, Rng};

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");
    let example = std::env::args().any(|arg| arg == "example");

    let input = if example {
        include_str!("example")
    } else {
        include_str!("input")
    };

    #[derive(Clone, Copy, Debug)]
    struct Ray<T> {
        pos: vec3<T>,
        vel: vec3<T>,
    }

    impl<T> Ray<T> {
        fn map<U>(self, f: impl Fn(T) -> U + Copy) -> Ray<U> {
            Ray {
                pos: self.pos.map(f),
                vel: self.vel.map(f),
            }
        }
    }

    let hailstones: Vec<Ray<i64>> = input
        .lines()
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();
            let parse = |s: &str| -> vec3<_> {
                let v: Vec<_> = s.split(',').map(|s| s.trim().parse().unwrap()).collect();
                let [x, y, z]: [i64; 3] = v.try_into().unwrap();
                vec3(x, y, z)
            };
            let pos = parse(pos);
            let vel = parse(vel);
            Ray { pos, vel }
        })
        .collect();

    if part1 {
        let mut answer = 0;

        let test_area: RangeInclusive<i64> = if example {
            7..=27
        } else {
            200000000000000..=400000000000000
        };
        let test_area = Aabb2 {
            min: vec2::splat(*test_area.start()),
            max: vec2::splat(*test_area.end()),
        }
        .map(|x| x as f64);
        for (a, b) in hailstones.iter().tuple_combinations() {
            let p1 = a.pos.xy();
            let v1 = a.vel.xy();
            let p2 = b.pos.xy();
            let v2 = b.vel.xy();

            fn intersect_time(
                p1: vec2<i64>,
                v1: vec2<i64>,
                p2: vec2<i64>,
                v2: vec2<i64>,
            ) -> Option<f64> {
                // p1 + v1 * t
                // skew(p - p2, v2) = 0
                // skew(p1 + v1 * t - p2, v2) = 0
                // skew(v1, v2) * t = skew(p2 - p1, v2)
                if vec2::skew(v1, v2) == 0 {
                    return None;
                }
                let t = vec2::skew(p2 - p1, v2) as f64 / vec2::skew(v1, v2) as f64;
                Some(t)
            }
            let Some(t1) = intersect_time(p1, v1, p2, v2) else {
                continue;
            };
            let t2 = intersect_time(p2, v2, p1, v1).unwrap();
            if t1 < 0.0 || t2 < 0.0 {
                continue;
            }
            let p = p1.map(|x| x as f64) + v1.map(|x| x as f64) * t1;
            if test_area.contains(p) {
                answer += 1;
            }
        }
        dbg!(answer);
    } else {
        // part2

        // time for RAY a to hit LINE b
        fn intersect_time(a: Ray<i64>, b: Ray<i64>) -> Option<f64> {
            let a = a.map(|x| x as f64);
            let b = b.map(|x| x as f64);
            // a.pos + a.vel * t
            // cross(p - b.pos, v2) = 0
            // cross(a.pos + a.vel * t - b.pos, b.vel) = 0
            // cross(a.vel, b.vel) * t = cross(b.pos - a.pos, b.vel)
            // t = cross(b.pos - a.pos, b.vel) / cross(a.vel, b.vel)
            let den = vec3::cross(a.vel, b.vel);
            if den == vec3::ZERO {
                return None;
            }
            let nom = vec3::cross(b.pos - a.pos, b.vel);
            let t = nom / den;
            dbg!(t);
            t.iter().all_equal().then_some(t.x)
        }

        // t = cross(b.pos - a.pos, b.vel) / cross(a.vel, b.vel)
        // t = cross(a.pos - b.pos, a.vel) / cross(b.vel, a.vel)
        // cross(a.pos - b.pos, b.vel) = cross(a.pos - b.pos, a.vel)
        // cross(a.pos - b.pos, b.vel) - cross(a.pos - b.pos, a.vel) = 0
        // cross(a.pos - b.pos, a.vel - b.vel).len() = 0
        // cross(a.pos - c.pos, a.vel - c.vel) = 0

        // p[i] + v[i] * t[i] = p + v * t[i]

        let f = |me: Ray<f64>| -> f64 {
            hailstones
                .iter()
                .take(5)
                .map(|stone| {
                    vec3::cross(
                        me.pos - stone.pos.map(|x| x as f64),
                        me.vel - stone.vel.map(|x| x as f64),
                    )
                    .len_sqr()
                })
                .sum()
        };

        fn rng_descent<const N: usize>(
            initial_guess: [f64; N],
            mut radius: [f64; N],
            f: impl Fn([f64; N]) -> f64,
        ) -> [f64; N] {
            let mut rng = thread_rng();
            let mut current_best_guess = initial_guess;
            while radius.iter().copied().max_by(f64::total_cmp).unwrap() > 0.1 {
                loop {
                    let guesses = std::iter::repeat_with::<[f64; N], _>(|| {
                        std::array::from_fn(|i| {
                            current_best_guess[i] + rng.gen_range(-radius[i]..=radius[i])
                        })
                    })
                    .take(100000);
                    let new_best_guess = guesses.min_by_key(|&p| r64(f(p))).unwrap();
                    let improvement = f(current_best_guess).max(0.1) - f(new_best_guess);
                    // if radius < 1.0 {
                    // dbg!(improvement);
                    // }
                    if improvement < 0.1 {
                        break;
                    }
                    if improvement > 0.0 {
                        current_best_guess = new_best_guess;
                    }
                }
                for x in &mut radius {
                    *x *= 0.9;
                }
            }
            current_best_guess
        }

        loop {
            fn find_range(hailstones: &[Ray<i64>], f: impl Fn(&Ray<i64>) -> i64) -> Range<i64> {
                let f = &f;
                hailstones.iter().map(f).min().unwrap()..hailstones.iter().map(f).max().unwrap()
            }
            let guess = Ray {
                pos: {
                    let [x, y, z] = std::array::from_fn(|i| {
                        thread_rng().gen_range(find_range(&hailstones, |stone| stone.pos[i]))
                    });
                    vec3(x, y, z).map(|x| x as f64)
                },
                vel: vec3::ZERO,
            };
            let [rx, ry, rz] = std::array::from_fn(|i| {
                let range = find_range(&hailstones, |stone| stone.pos[i]);
                (range.end - range.start) as f64
            });
            let [rvx, rvy, rvz] = std::array::from_fn(|i| {
                let range = find_range(&hailstones, |stone| stone.vel[i]);
                (range.end - range.start) as f64
            });
            let [x, y, z, vx, vy, vz] = rng_descent(
                [
                    guess.pos.x,
                    guess.pos.y,
                    guess.pos.z,
                    guess.vel.x,
                    guess.vel.y,
                    guess.vel.z,
                ],
                [rx, ry, rz, rvx, rvy, rvz],
                |[x, y, z, vx, vy, vz]| {
                    f(Ray {
                        pos: vec3(x, y, z),
                        vel: vec3(vx, vy, vz),
                    })
                },
            );
            let me = Ray {
                pos: vec3(x, y, z),
                vel: vec3(vx, vy, vz),
            };
            // let me = Ray {
            //     pos: vec3(24.0, 13.0, 10.0),
            //     vel: vec3(-3.0, 1.0, 2.0),
            // };
            println!("before rounding");
            dbg!(me, f(me));
            let me = me.map(|x| x.round());
            println!("after rounding");
            dbg!(me, f(me));
            let me = me.map(|x| x as i64);

            let mut actually_the_answer = true;
            for &stone in &hailstones {
                fn check_hits_in_future(ray: Ray<i64>, stone: Ray<i64>) -> bool {
                    let rel_target = stone.pos - ray.pos;
                    let rel_vel = ray.vel - stone.vel;
                    vec3::cross(rel_target, rel_vel) == vec3::ZERO
                        && vec3::dot(rel_target, rel_vel) >= 0
                }
                if !check_hits_in_future(me, stone) || !check_hits_in_future(stone, me) {
                    actually_the_answer = false;
                    break;
                }
            }
            if actually_the_answer {
                println!("WOOOOO");
                let answer = me.pos.x + me.pos.y + me.pos.z;
                println!("{answer}");
                break;
            }
        }
    }
}
