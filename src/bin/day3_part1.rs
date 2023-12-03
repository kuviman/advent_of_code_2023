fn main() {
    let map = std::fs::read_to_string("input/day3").unwrap();
    let map: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();

    let mut answer = 0;

    for row in 0..map.len() {
        let mut col = 0;
        while col < map[row].len() {
            if map[row][col].is_ascii_digit() {
                let mut number: u32 = 0;
                let start = col;
                while let Some(digit) = map[row].get(col).copied().and_then(|c| c.to_digit(10)) {
                    number = number.checked_mul(10).unwrap().checked_add(digit).unwrap();
                    col += 1;
                }
                let end = col;
                let mut part = false;
                'is_part: for other_row in (row.max(1) - 1)..=(row + 1).min(map.len() - 1) {
                    for other_col in (start.max(1) - 1)..(end + 1).min(map[row].len()) {
                        let c = map[other_row][other_col];
                        if !c.is_ascii_digit() && c != '.' {
                            part = true;
                            break 'is_part;
                        }
                    }
                }
                if part {
                    answer += number;
                }
            }
            col += 1;
        }
    }

    println!("{answer}");
}
