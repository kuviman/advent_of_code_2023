use std::{collections::HashMap, str::FromStr, sync::Arc};

fn main() {
    let part1 = !std::env::args().any(|arg| arg == "part2");

    let input = if std::env::args().any(|arg| arg == "example") {
        include_str!("example")
    } else {
        include_str!("input")
    };

    #[derive(Clone, Debug)]
    enum Destination {
        Accepted,
        Rejected,
        Workflow(Arc<str>),
    }

    impl FromStr for Destination {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "A" => Self::Accepted,
                "R" => Self::Rejected,
                _ => Self::Workflow(s.into()),
            })
        }
    }

    #[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
    enum Category {
        ExtremelyCoolLooking,
        Musical,
        Aerodynamic,
        Shiny,
    }

    impl FromStr for Category {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "x" => Self::ExtremelyCoolLooking,
                "m" => Self::Musical,
                "a" => Self::Aerodynamic,
                "s" => Self::Shiny,
                _ => unreachable!(),
            })
        }
    }

    #[derive(Debug)]
    enum Operator {
        Less,
        Greater,
    }

    impl Operator {
        fn matches(&self, cmp: std::cmp::Ordering) -> bool {
            #[allow(clippy::match_like_matches_macro)]
            match (self, cmp) {
                (Self::Less, std::cmp::Ordering::Less) => true,
                (Self::Greater, std::cmp::Ordering::Greater) => true,
                _ => false,
            }
        }
    }

    impl FromStr for Operator {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                ">" => Self::Greater,
                "<" => Self::Less,
                _ => unreachable!(),
            })
        }
    }

    type Value = u64;

    #[derive(Debug)]
    struct Condition {
        category: Category,
        operator: Operator,
        constant: Value,
    }

    impl Condition {
        fn check(&self, part: &Part) -> bool {
            let part_value = part.categories[&self.category];
            self.operator.matches(part_value.cmp(&self.constant))
        }
    }

    impl FromStr for Condition {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self {
                category: s[..1].parse().unwrap(),
                operator: s[1..2].parse().unwrap(),
                constant: s[2..].parse().unwrap(),
            })
        }
    }

    struct Rule {
        condition: Option<Condition>,
        dest: Destination,
    }

    impl Rule {
        fn apply(&self, part: &Part) -> Option<Destination> {
            self.condition
                .as_ref()
                .map_or(true, |condition| condition.check(part))
                .then(|| self.dest.clone())
        }
    }

    impl FromStr for Rule {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s.find(':') {
                Some(colon) => Self {
                    condition: Some(s[..colon].parse().unwrap()),
                    dest: s[colon + 1..].parse().unwrap(),
                },
                None => Self {
                    condition: None,
                    dest: s.parse().unwrap(),
                },
            })
        }
    }

    struct Workflow {
        name: Arc<str>,
        rules: Vec<Rule>,
    }

    impl Workflow {
        fn apply(&self, part: &Part) -> Destination {
            for rule in &self.rules {
                if let Some(dest) = rule.apply(part) {
                    return dest;
                }
            }
            unreachable!()
        }
    }

    impl FromStr for Workflow {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let open = s.find('{').unwrap();
            let name = s[..open].into();
            let s = s[open + 1..].strip_suffix('}').unwrap();
            Ok(Self {
                name,
                rules: s.split(',').map(|s| s.parse().unwrap()).collect(),
            })
        }
    }

    struct Part {
        categories: HashMap<Category, Value>,
    }

    impl FromStr for Part {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
            Ok(Self {
                categories: s
                    .split(',')
                    .map(|s| {
                        let (category, value) = s.split_once('=').unwrap();
                        (category.parse().unwrap(), value.parse().unwrap())
                    })
                    .collect(),
            })
        }
    }

    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<Arc<str>, Workflow> = workflows
        .lines()
        .map(|s| {
            let workflow: Workflow = s.parse().unwrap();
            (workflow.name.clone(), workflow)
        })
        .collect();

    let parts: Vec<Part> = parts.lines().map(|s| s.parse().unwrap()).collect();

    trait IteratorExt: Iterator + Sized
    where
        Self::Item: std::hash::Hash + Eq,
    {
        fn count_occurences(self) -> HashMap<Self::Item, usize> {
            let mut result = HashMap::new();
            for item in self {
                *result.entry(item).or_default() += 1;
            }
            result
        }
    }

    impl<T: Iterator> IteratorExt for T where T::Item: std::hash::Hash + Eq {}

    let total_rules: HashMap<Category, usize> = workflows
        .values()
        .flat_map(|workflow| {
            workflow
                .rules
                .iter()
                .filter_map(|rule| rule.condition.as_ref())
                .map(|condition| condition.category)
        })
        .count_occurences();
    dbg!(total_rules);

    if part1 {
        let mut answer = 0;
        for part in parts {
            let mut workflow: Arc<str> = "in".into();
            let accepted = loop {
                match workflows[&workflow].apply(&part) {
                    Destination::Accepted => break true,
                    Destination::Rejected => break false,
                    Destination::Workflow(next) => {
                        workflow = next;
                    }
                }
            };
            if accepted {
                answer += part.categories.values().copied().sum::<Value>();
            }
        }
        println!("{answer}");
    } else {
        #[derive(Debug, Copy, Clone)]
        struct Range {
            start: Value,
            end: Value,
        }
        impl Range {
            fn valid(&self) -> bool {
                self.start <= self.end
            }
        }
        #[derive(Debug, Clone)]
        struct Hypercube {
            categories: HashMap<Category, Range>,
        }
        impl Hypercube {
            fn volume(&self) -> Value {
                self.categories
                    .values()
                    .map(|range| range.end - range.start + 1)
                    .product()
            }
        }
        fn solve(
            workflows: &HashMap<Arc<str>, Workflow>,
            workflow: &str,
            mut hypercube: Hypercube,
        ) -> Value {
            let workflow = &workflows[workflow];
            let mut result = 0;
            for rule in &workflow.rules {
                let (passing, not_passing) = match &rule.condition {
                    Some(condition) => {
                        let current_range = hypercube.categories[&condition.category];
                        let (passing, not_passing) = match condition.operator {
                            Operator::Less => (
                                Range {
                                    start: current_range.start,
                                    end: condition.constant - 1,
                                },
                                Range {
                                    start: condition.constant,
                                    end: current_range.end,
                                },
                            ),
                            Operator::Greater => (
                                Range {
                                    start: condition.constant + 1,
                                    end: current_range.end,
                                },
                                Range {
                                    start: current_range.start,
                                    end: condition.constant,
                                },
                            ),
                        };
                        let range = |range: Range| -> Option<Hypercube> {
                            if range.valid() {
                                let mut new_hypercube = hypercube.clone();
                                new_hypercube.categories.insert(condition.category, range);
                                Some(new_hypercube)
                            } else {
                                None
                            }
                        };
                        (range(passing), range(not_passing))
                    }
                    None => (Some(hypercube.clone()), None),
                };
                if let Some(passing) = passing {
                    match &rule.dest {
                        Destination::Accepted => result += passing.volume(),
                        Destination::Rejected => {}
                        Destination::Workflow(next) => result += solve(workflows, next, passing),
                    }
                }
                match not_passing {
                    Some(not_passing) => hypercube = not_passing,
                    None => break,
                }
            }
            result
        }
        let answer = solve(
            &workflows,
            "in",
            Hypercube {
                categories: [
                    Category::ExtremelyCoolLooking,
                    Category::Aerodynamic,
                    Category::Musical,
                    Category::Shiny,
                ]
                .into_iter()
                .map(|category| {
                    (
                        category,
                        Range {
                            start: 1,
                            end: 4000,
                        },
                    )
                })
                .collect(),
            },
        );
        println!("{answer}");
    }
}
