use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day4").unwrap();
    let mut answer = 0;
    for line in input.lines() {
        let actual_data = &line[line.find(':').unwrap() + 1..];
        let mut parts = actual_data.split('|');
        let winning_numbers: HashSet<&str> =
            parts.next().unwrap().split_ascii_whitespace().collect();
        let my_numbers = parts.next().unwrap().split_ascii_whitespace();

        let hits = my_numbers
            .filter(|number| winning_numbers.contains(number))
            .count();
        if hits != 0 {
            answer += 2u64.pow(hits as u32 - 1);
        }
    }
    println!("{answer}");
}
