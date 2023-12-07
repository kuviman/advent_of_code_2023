use std::collections::{HashMap, HashSet};

fn main() {
    const POSSIBLE_CARD: &str = "23456789TJQKA";
    #[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Clone)]
    struct Card(u8);
    impl Card {
        fn from_char(c: char) -> Self {
            Self(
                POSSIBLE_CARD
                    .chars()
                    .position(|possible| possible == c)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
        }
    }
    type Hand = [Card; 5];
    let mut hands: Vec<(Hand, u64)> = {
        std::fs::read_to_string("input/day7")
            .unwrap()
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let cards = parts
                    .next()
                    .unwrap()
                    .chars()
                    .map(Card::from_char)
                    .collect::<Vec<Card>>()
                    .try_into()
                    .ok()
                    .unwrap();
                let bid = parts.next().unwrap().parse().unwrap();
                (cards, bid)
            })
            .collect()
    };
    let unique_hands = hands
        .iter()
        .map(|(hand, _bid)| hand)
        .collect::<HashSet<&Hand>>();
    assert_eq!(unique_hands.len(), hands.len());

    #[derive(PartialEq, PartialOrd, Eq, Ord)]
    enum Rank {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }
    fn rank(hand: &Hand) -> Rank {
        let mut counts = HashMap::<&Card, usize>::new();
        for card in hand {
            *counts.entry(card).or_default() += 1;
        }
        let mut counts: Vec<usize> = counts.into_values().filter(|&count| count > 1).collect();
        counts.sort();
        match counts.as_slice() {
            [] => Rank::HighCard,
            [2] => Rank::OnePair,
            [2, 2] => Rank::TwoPair,
            [3] => Rank::ThreeOfAKind,
            [2, 3] => Rank::FullHouse,
            [4] => Rank::FourOfAKind,
            [5] => Rank::FiveOfAKind,
            _ => unreachable!(),
        }
    }

    hands.sort_by_key(|(hand, _bid)| (rank(hand), hand.clone()));

    let answer: u64 = hands
        .iter()
        .enumerate()
        .map(|(index, (_hand, bid))| {
            let rank = index + 1;
            rank as u64 * bid
        })
        .sum();
    println!("{answer}");
}
