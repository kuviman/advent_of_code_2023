use std::{
    collections::{HashMap, HashSet},
    ops::{Div, Mul},
    process::Output,
};

use gcd::Gcd;

fn lcm<T: Gcd + Mul<Output = T> + Div<Output = T> + Copy>(a: T, b: T) -> T {
    a * b / a.gcd(b)
}

fn main() {
    let input = std::fs::read_to_string("input/day8").unwrap();

    enum Command {
        Left,
        Right,
    }

    impl Command {
        fn from_char(c: char) -> Option<Self> {
            Some(match c {
                'L' => Self::Left,
                'R' => Self::Right,
                _ => return None,
            })
        }
    }

    let mut lines = input.lines();
    let commands: Vec<Command> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| Command::from_char(c).unwrap())
        .collect();

    let nodes: HashMap<&str, (&str, &str)> = {
        assert!(lines.next().unwrap().is_empty());
        lines
            .map(|line| {
                let (name, tuple) = line.split_once(" = ").unwrap();
                let (left, right) = tuple
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split_once(", ")
                    .unwrap();
                (name, (left, right))
            })
            .collect()
    };

    println!("nodes: {}", nodes.len());
    println!("commands: {}", commands.len());

    let initial_nodes: Vec<&str> = nodes
        .keys()
        .copied()
        .filter(|node| node.ends_with('A'))
        .collect();

    println!("initial nodes: {}", initial_nodes.len());

    #[derive(Debug)]
    struct Cycle {
        end_with_z: Vec<bool>,
        cycle_start: usize,
    }
    let find_cycle = |node: &str| -> Cycle {
        let mut end_with_z = Vec::new();
        let mut visited = HashMap::<(&str, usize), usize>::new();
        let mut current_node = node;
        let mut current_command_index = 0;
        while !visited.contains_key(&(current_node, current_command_index)) {
            visited.insert((current_node, current_command_index), end_with_z.len());
            end_with_z.push(current_node.ends_with('Z'));
            let command = &commands[current_command_index];
            current_command_index += 1;
            current_command_index %= commands.len();
            let &(left, right) = nodes.get(current_node).unwrap();
            current_node = match command {
                Command::Left => left,
                Command::Right => right,
            }
        }
        Cycle {
            end_with_z,
            cycle_start: visited[&(current_node, current_command_index)],
        }
    };

    for &node in &initial_nodes {
        let cycle = find_cycle(node);
        println!(
            "Cycle for {node:?} = {}/{}/{}, end_with_z={}",
            cycle.end_with_z.len(),
            cycle.cycle_start,
            cycle.end_with_z.len() - cycle.cycle_start,
            cycle.end_with_z.iter().filter(|&&b| b).count(),
        );
    }

    // cycle.start + (step - cycle.start) % cycle.len == z_pos

    /// valid steps = start + period * t, where t >= 0
    struct ValidSteps {
        start: usize,
        period: usize,
    }

    let mut valid_steps = ValidSteps {
        start: 0,
        period: 1,
    };

    fn merge(a: ValidSteps, b: ValidSteps) -> ValidSteps {
        // s1 + p1 * k1 = s2 + p2 * k2
        // k1 = (s2 + p2 * k2 - s1) / p1
        let (a, b) = if a.period > b.period { (a, b) } else { (b, a) };
        let mut k1 = 0;
        let new_start = loop {
            let nom = a.start as i64 + a.period as i64 * k1 as i64 - b.start as i64;
            if nom % b.period as i64 == 0 {
                let k2 = nom / b.period as i64;
                if k2 >= 0 {
                    break a.start + a.period * k1;
                }
            }
            k1 += 1;
        };

        ValidSteps {
            start: new_start,
            period: lcm(a.period, b.period),
        }
    }

    for &node in &initial_nodes {
        let cycle = find_cycle(node);
        let z_pos = cycle
            .end_with_z
            .iter()
            .copied()
            .position(|end_with_z| end_with_z)
            .unwrap();
        assert!(z_pos >= cycle.cycle_start);
        let current_valid_steps = ValidSteps {
            start: z_pos,
            period: cycle.end_with_z.len() - cycle.cycle_start,
        };

        valid_steps = merge(valid_steps, current_valid_steps);
    }

    println!("{}", valid_steps.start);
}
