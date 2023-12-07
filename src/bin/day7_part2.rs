use std::collections::{HashMap, HashSet};

fn main() {
    const POSSIBLE_CARD: &str = "J23456789TQKA";
    #[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
    struct Card(u8);
    impl std::fmt::Debug for Card {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let pos = self.0 as usize;
            write!(f, "{}", &POSSIBLE_CARD[pos..pos + 1])
        }
    }
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

    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
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
        fn actual_rank(hand: &Hand) -> Rank {
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

        let mut counts = HashMap::<&Card, usize>::new();
        for card in hand {
            *counts.entry(card).or_default() += 1;
        }

        let mut new_hand = hand.clone();
        if let Some(replacement) = counts
            .into_iter()
            .filter(|(card, _)| **card != Card::from_char('J'))
            .max_by_key(|(_card, count)| *count)
            .map(|(card, _)| card.clone())
        {
            for card in &mut new_hand {
                if *card == Card::from_char('J') {
                    *card = replacement.clone();
                }
            }
        }

        actual_rank(&new_hand)
    }

    hands.sort_by_key(|(hand, _bid)| (rank(hand), hand.clone()));

    let answer: u64 = hands
        .iter()
        .enumerate()
        .map(|(index, (hand, bid))| {
            // println!("rank of {hand:?} = {:?}", rank(hand));
            let rank = index + 1;
            rank as u64 * bid
        })
        .sum();
    println!("{answer}");
}
