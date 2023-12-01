fn main() {
    let input = std::fs::read_to_string("input/day1").unwrap();
    let mut result: u64 = 0;
    for line in input.lines() {
        const STRING_DIGITS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let digits = STRING_DIGITS
            .into_iter()
            .enumerate()
            .flat_map(|(i, digit_str)| {
                let digit = i as u32 + 1;
                let str_matches = line.match_indices(digit_str);
                let char_matches = line.match_indices(char::from_digit(digit, 10).unwrap());
                char_matches
                    .chain(str_matches) // comment for part1 :)
                    .map(move |(i, _match)| (digit, i))
            });
        let first_digit = digits.clone().min_by_key(|&(_, pos)| pos).unwrap().0;
        let last_digit = digits.max_by_key(|&(_, pos)| pos).unwrap().0;
        let calibration_value = first_digit * 10 + last_digit;
        result = result.checked_add(calibration_value.into()).unwrap();
    }
    println!("{result}")
}
