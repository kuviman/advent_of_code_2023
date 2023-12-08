use std::collections::HashMap;

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

    let mut current_node = "AAA";
    let mut commands = commands.iter().cycle();
    let mut answer = 0;
    while current_node != "ZZZ" {
        let &(left, right) = nodes.get(current_node).unwrap();
        current_node = match commands.next().unwrap() {
            Command::Left => left,
            Command::Right => right,
        };
        answer += 1;
    }
    println!("{answer}");
}
