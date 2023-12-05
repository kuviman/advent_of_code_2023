use std::{
    collections::{BTreeMap, HashMap},
    ops::Range,
};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day5").unwrap();
    let mut lines = input.lines();

    let initial_seeds: Vec<Range<u64>> = {
        let line = lines.next().unwrap();
        let data_str = line.strip_prefix("seeds: ").unwrap();
        data_str
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .tuples()
            .map(|(start, len)| start..start + len)
            .collect()
    };

    type Ranges = BTreeMap<u64, (u64, u64)>;

    let mut mappings = HashMap::<&str, HashMap<&str, Ranges>>::new();

    let mut current_map: Option<(&str, &str, Ranges)> = None;
    for line in lines.chain(std::iter::once("")) {
        if line.is_empty() {
            if let Some((from, to, ranges)) = current_map.take() {
                let old = mappings.entry(from).or_default().insert(to, ranges);
                assert!(old.is_none());
            }
            continue;
        }
        match &mut current_map {
            Some((_from, _to, ranges)) => {
                let mut parts = line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<u64>().unwrap());
                let dest = parts.next().unwrap();
                let start = parts.next().unwrap();
                let len = parts.next().unwrap();
                assert!(parts.next().is_none());
                let old = ranges.insert(start, (dest, len));
                assert!(old.is_none());
            }
            None => {
                let (from, rest) = line.trim().split_once("-to-").unwrap();
                let to = rest.strip_suffix(" map:").unwrap();
                current_map = Some((from, to, Default::default()));
            }
        }
    }

    fn solve(range: Range<u64>, typ: &str, mappings: &HashMap<&str, HashMap<&str, Ranges>>) -> u64 {
        let Some(maps) = mappings.get(typ) else {
            assert_eq!(typ, "location");
            return range.start;
        };
        assert_eq!(maps.len(), 1);
        let (convert_into, ranges) = maps.iter().next().unwrap();

        // let Some(first) = ranges.range(..=range.start).last() else {
        //     return solve(range, convert_into, mappings);
        // };

        let mut last_processed = range.start;

        let mut answer = None;
        let mut update_answer = |x: u64| {
            answer = Some(answer.map_or(x, |y| std::cmp::min(x, y)));
        };

        // assert!(*first.0 <= range.end);
        // for (&start, &(dest, len)) in ranges.range(*first.0..range.end) {
        for (&start, &(dest, len)) in ranges.range(..range.end) {
            if start > last_processed {
                update_answer(solve(last_processed..start, convert_into, mappings));
            }

            let intersection_start = std::cmp::max(start, range.start);
            let intersection_end = std::cmp::min(start + len, range.end);

            if intersection_start < intersection_end {
                update_answer(solve(
                    intersection_start + dest - start..intersection_end + dest - start,
                    convert_into,
                    mappings,
                ));
            }

            last_processed = last_processed.max(intersection_end);
        }

        if range.end > last_processed {
            update_answer(solve(last_processed..range.end, convert_into, mappings));
        }

        answer.unwrap()
    }

    let answer = initial_seeds
        .into_iter()
        .map(|range| solve(range, "seed", &mappings))
        .min()
        .unwrap();
    println!("{answer}");
}
