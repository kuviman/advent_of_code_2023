fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");

    let input = if std::env::args().any(|arg| arg == "example") {
        include_str!("example")
    } else {
        include_str!("input")
    };

    fn hash(s: &str) -> u8 {
        let mut result: u8 = 0;
        for c in s.chars() {
            if c == '\n' {
                continue;
            }
            result = result.overflowing_add(c as u8).0.overflowing_mul(17).0;
        }
        // println!("{s} => {result}");
        result
    }

    if part1 {
        let answer: u64 = input.split(',').map(hash).map(|x| x as u64).sum();
        println!("{answer}");
    } else {
        struct Lens<'a> {
            label: &'a str,
            focal_length: u64,
        }

        struct HashMap<'a> {
            boxes: [Vec<Lens<'a>>; 256],
        }
        impl<'a> HashMap<'a> {
            fn new() -> Self {
                Self {
                    boxes: std::array::from_fn(|_| vec![]),
                }
            }
            fn remove(&mut self, label: &'a str) -> Option<Lens<'a>> {
                let box_index = hash(label) as usize;
                let the_box = &mut self.boxes[box_index];
                let lens_index = the_box.iter().position(|lens| lens.label == label)?;
                Some(the_box.remove(lens_index))
            }
            fn insert(&mut self, lens: Lens<'a>) {
                let box_index = hash(lens.label) as usize;
                let the_box = &mut self.boxes[box_index];
                if let Some(existing_lens) = the_box
                    .iter_mut()
                    .find(|existing| existing.label == lens.label)
                {
                    *existing_lens = lens;
                } else {
                    the_box.push(lens);
                }
            }
        }

        let mut map = HashMap::new();

        for command in input.split(',') {
            let command = command.trim();
            if let Some(label) = command.strip_suffix('-') {
                map.remove(label);
            } else {
                let (label, focal_length) = command.split_once('=').unwrap();
                let lens = Lens {
                    label,
                    focal_length: focal_length.parse().unwrap(),
                };
                map.insert(lens);
            }
        }

        let answer: u64 = map
            .boxes
            .iter()
            .enumerate()
            .flat_map(|(box_index, the_box)| {
                the_box.iter().enumerate().map(move |(lens_index, lens)| {
                    (box_index as u64 + 1) * (lens_index as u64 + 1) * lens.focal_length
                })
            })
            .sum();
        println!("{answer}");
    }
}
