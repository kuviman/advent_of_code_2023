use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input/day4").unwrap();
    let mut answer = 0;
    let mut next_cards = Vec::<u64>::new();
    for line in input.lines().rev() {
        let actual_data = &line[line.find(':').unwrap() + 1..];
        let mut parts = actual_data.split('|');
        let winning_numbers: HashSet<&str> =
            parts.next().unwrap().split_ascii_whitespace().collect();
        let my_numbers = parts.next().unwrap().split_ascii_whitespace();

        let hits = my_numbers
            .filter(|number| winning_numbers.contains(number))
            .count();

        let mut this_number = 1;
        for card in next_cards.iter().rev().copied().take(hits) {
            this_number += card;
        }
        next_cards.push(this_number);
        answer += this_number;
    }
    println!("{answer}");
}
