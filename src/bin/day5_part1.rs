use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = std::fs::read_to_string("input/day5").unwrap();
    let mut lines = input.lines();

    let initial_seeds: Vec<u64> = {
        let line = lines.next().unwrap();
        let data_str = line.strip_prefix("seeds: ").unwrap();
        data_str
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
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

    let answer = initial_seeds
        .into_iter()
        .map(|seed| {
            let mut current_type = "seed";
            let mut current_id = seed;
            while let Some(maps) = mappings.get(current_type) {
                assert!(maps.len() == 1);
                let (next_type, ranges) = maps.iter().next().unwrap();
                current_id = ranges
                    .range(..=current_id)
                    .last()
                    .and_then(|(start, (dest, len))| {
                        if current_id < start + len {
                            Some(dest + (current_id - start))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(current_id);
                current_type = next_type;
            }
            assert_eq!(current_type, "location");
            current_id
        })
        .min()
        .unwrap();
    println!("{answer}");
}
