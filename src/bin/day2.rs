use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/day2").unwrap();

    let mut answer: u64 = 0;

    for line in input.lines() {
        let line = line.strip_prefix("Game ").unwrap();
        let colon = line.find(':').unwrap();
        let game_id: u64 = line[..colon].parse().unwrap();

        let subsets: Vec<HashMap<&str, u64>> = line[colon + 1..]
            .split(';')
            .map(|s| {
                s.split(',')
                    .map(|s| {
                        let mut parts = s.split_whitespace();
                        let number: u64 = parts.next().unwrap().parse().unwrap();
                        let color: &str = parts.next().unwrap();
                        (color, number)
                    })
                    .collect()
            })
            .collect();

        let mut minimum_set = HashMap::<&str, u64>::new();
        for subset in subsets {
            for (color, number) in subset {
                let minimum_number = minimum_set.entry(color).or_default();
                *minimum_number = (*minimum_number).max(number);
            }
        }

        let minimum_set_power: u64 = minimum_set.into_values().product();
        answer += minimum_set_power;
    }

    println!("{answer}")
}
