use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    ops::{Div, Mul},
    sync::Arc,
};

use itertools::Itertools;

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");

    let input = if std::env::args().any(|arg| arg == "example") {
        include_str!("example")
    } else {
        include_str!("input")
    };

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
    enum Pulse {
        Low,
        High,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum ModuleType {
        FlipFlop(bool),
        Conjunction(BTreeMap<Arc<str>, Pulse>),
        Broadcast,
    }

    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    struct Module {
        typ: ModuleType,
        outputs: Vec<Arc<str>>,
    }

    let mut modules: BTreeMap<Arc<str>, Module> = input
        .lines()
        .map(|line| {
            let (type_name, outputs) = line.split_once(" -> ").unwrap();
            let (typ, name) = if type_name == "broadcaster" {
                (ModuleType::Broadcast, type_name)
            } else if let Some(name) = type_name.strip_prefix('%') {
                (ModuleType::FlipFlop(false), name)
            } else if let Some(name) = type_name.strip_prefix('&') {
                (ModuleType::Conjunction(BTreeMap::new()), name)
            } else {
                unreachable!()
            };
            let outputs = outputs.split(", ").map(|s| s.into()).collect();
            // println!("{name:?} -> {outputs:?}");
            (name.into(), Module { typ, outputs })
        })
        .collect();

    for (name, outputs) in modules
        .iter()
        .map(|(name, module)| (name.clone(), module.outputs.clone()))
        .collect_vec()
    {
        for output in outputs {
            if let Some(ModuleType::Conjunction(inputs)) =
                &mut modules.get_mut(&output).map(|module| &mut module.typ)
            {
                inputs.insert(name.clone(), Pulse::Low);
            }
        }
    }

    fn simulate(
        from: &str,
        node: &str,
        pulse: Pulse,
        modules: &mut BTreeMap<Arc<str>, Module>,
        mut f: impl FnMut(&str, Pulse),
    ) {
        let mut queue =
            VecDeque::<(Arc<str>, Arc<str>, Pulse)>::from_iter([(from.into(), node.into(), pulse)]);
        while let Some((from, name, pulse)) = queue.pop_front() {
            // println!("{from} -{pulse:?}-> {name}");
            f(&name, pulse);
            let Some(module) = modules.get_mut(&name) else {
                continue;
            };
            let send = match &mut module.typ {
                ModuleType::FlipFlop(state) => match pulse {
                    Pulse::Low => {
                        *state = !*state;
                        Some(match *state {
                            true => Pulse::High,
                            false => Pulse::Low,
                        })
                    }
                    Pulse::High => None,
                },
                ModuleType::Conjunction(state) => {
                    state.insert(from, pulse);
                    Some(match state.values().all(|&pulse| pulse == Pulse::High) {
                        true => Pulse::Low,
                        false => Pulse::High,
                    })
                }
                ModuleType::Broadcast => Some(pulse),
            };
            if let Some(send) = send {
                for output in &module.outputs {
                    queue.push_back((name.clone(), output.clone(), send));
                }
            }
        }
    }

    if part1 {
        let mut lows = 0;
        let mut highs = 0;
        for _ in 0..1000 {
            simulate(
                "button",
                "broadcaster",
                Pulse::Low,
                &mut modules,
                |_, pulse| match pulse {
                    Pulse::Low => lows += 1,
                    Pulse::High => highs += 1,
                },
            );
        }
        dbg!(lows);
        dbg!(highs);
        let answer = lows * highs;
        println!("{answer}");
    } else {
        let final_conjuntion = "hp";
        assert!(matches!(
            &modules[final_conjuntion].typ,
            ModuleType::Conjunction(..),
        ));
        assert!(modules[final_conjuntion]
            .outputs
            .iter()
            .all(|output| &**output == "rx"));

        // === COPY PASTED FROM DAY 8
        use gcd::Gcd;

        fn lcm<T: Gcd + Mul<Output = T> + Div<Output = T> + Copy>(a: T, b: T) -> T {
            a * b / a.gcd(b)
        }
        /// valid steps = start + period * t, where t >= 0
        struct ValidSteps {
            start: usize,
            period: usize,
        }

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
        // === END OF COPYPASTA

        let mut all_highs = ValidSteps {
            start: 1,
            period: 1,
        };

        for output in modules["broadcaster"].outputs.clone() {
            let mut modules = modules.clone();
            let mut hp_high_pulses = Vec::new();
            let mut been_there = HashMap::new();
            let mut button_presses = 0;
            let loop_len = loop {
                if let Some(prev) = been_there.insert(modules.clone(), button_presses) {
                    break button_presses - prev;
                }
                button_presses += 1;
                simulate(
                    "broadcaster",
                    &output,
                    Pulse::Low,
                    &mut modules,
                    |name, pulse| {
                        if name == "hp" && pulse == Pulse::High {
                            hp_high_pulses.push(button_presses);
                        }
                    },
                );
            };
            dbg!(loop_len);
            assert_eq!(hp_high_pulses.len(), 1);
            let high_pulse = hp_high_pulses[0];
            for _ in 0..loop_len {
                button_presses += 1;
                simulate(
                    "broadcaster",
                    &output,
                    Pulse::Low,
                    &mut modules,
                    |name, pulse| {
                        if name == "hp" && pulse == Pulse::High {
                            assert_eq!(button_presses, high_pulse + loop_len);
                        }
                    },
                );
            }
            all_highs = merge(
                all_highs,
                ValidSteps {
                    start: high_pulse,
                    period: loop_len,
                },
            );
        }

        let answer = all_highs.start;
        println!("{answer}");
    }
}
