fn main() {
    type Map = Vec<Vec<char>>;
    let input: Vec<Map> = std::fs::read_to_string("input/day13")
        .unwrap()
        .split("\n\n")
        .map(|map| map.lines().map(|line| line.chars().collect()).collect())
        .collect();

    fn solve(map: &Map) -> u64 {
        fn solve_col(rows: usize, cols: usize, map: impl Fn(usize, usize) -> char) -> u64 {
            let mut answer = 0;
            let mirrored = |mirror_around_col: usize| -> bool {
                for col in 0..mirror_around_col {
                    let mirrored_col = mirror_around_col * 2 - 1 - col;
                    if mirrored_col < cols {
                        for row in 0..rows {
                            if map(row, col) != map(row, mirrored_col) {
                                return false;
                            }
                        }
                    }
                }
                true
            };
            for mirror_col in 1..cols {
                if mirrored(mirror_col) {
                    answer += mirror_col as u64;
                }
            }
            answer
        }
        solve_col(map.len(), map[0].len(), |row, col| map[row][col])
            + solve_col(map[0].len(), map.len(), |col, row| map[row][col]) * 100
    }

    let answer: u64 = input.iter().map(solve).sum();
    println!("{answer}");
}
