fn solve_line(line: &str) -> u64 {
    let (map, groups) = line.split_once(' ').unwrap();

    enum State {
        Broken,
        Operational,
        Unknown,
    }

    struct Data {
        map: Vec<State>,
        groups: Vec<usize>,
    }

    impl Data {
        fn solve(&self, map_pos: usize, group_pos: usize, current_group_size: usize) -> u64 {
            if map_pos == self.map.len() {
                assert_eq!(current_group_size, 0);
                return if group_pos == self.groups.len() { 1 } else { 0 };
            }

            let mut current = 0;
            for broken in [false, true] {
                if !matches!(
                    (broken, &self.map[map_pos]),
                    (true, State::Unknown | State::Broken)
                        | (false, State::Unknown | State::Operational)
                ) {
                    continue;
                }
                if broken {
                    current += self.solve(map_pos + 1, group_pos, current_group_size + 1);
                } else if current_group_size == 0 {
                    current += self.solve(map_pos + 1, group_pos, 0);
                } else {
                    let needed = self.groups.get(group_pos).copied();
                    if Some(current_group_size) == needed {
                        current += self.solve(map_pos + 1, group_pos + 1, 0);
                    }
                }
            }
            current
        }
    }

    let data = Data {
        map: map
            .chars()
            .map(|c| match c {
                '#' => State::Broken,
                '.' => State::Operational,
                '?' => State::Unknown,
                _ => unreachable!(),
            })
            .chain(std::iter::once(State::Operational))
            .collect(),
        groups: groups.split(',').map(|s| s.parse().unwrap()).collect(),
    };

    let answer = data.solve(0, 0, 0);
    // println!("{line} = {answer}");
    answer
}

fn main() {
    let input = std::fs::read_to_string("input/day12").unwrap();
    let line_answers = input.lines().map(solve_line);

    let answer: u64 = line_answers.sum();
    println!("{answer}");
}
