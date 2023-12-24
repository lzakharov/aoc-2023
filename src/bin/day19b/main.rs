use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day19.txt").unwrap();
    let reader = io::BufReader::new(file);

    let workflows: HashMap<String, Workflow> = reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|mut name| {
            let rules = name.split_off(name.find('{').unwrap());
            (name, Workflow::parse(&rules))
        })
        .collect();

    let mut ranges: Vec<(PartRange, String)> = vec![(
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        String::from("in"),
    )];

    let mut result = 0;

    while !ranges.is_empty() {
        let mut next = vec![];

        for (range, name) in ranges.into_iter() {
            if let Some(workflow) = workflows.get(&name) {
                next.extend(workflow.apply(range));
            } else if name == "A" {
                result += range.value();
            }
        }

        ranges = next;
    }

    println!("{result}");
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(s: &str) -> Self {
        let rules = s[1..s.len() - 1].split(',').map(Rule::parse).collect();
        Workflow { rules }
    }

    fn apply(&self, range: PartRange) -> Vec<(PartRange, String)> {
        let mut result = vec![];
        let mut next = Some(range);
        let mut rules = self.rules.iter();

        while let Some(range) = next {
            let (v, range) = rules.next().unwrap().apply(range);
            if let Some(v) = v {
                if v.1 != "R" {
                    result.push(v);
                }
            }
            next = range;
        }

        result
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    name: String,
}

impl Rule {
    fn parse(s: &str) -> Self {
        if s.contains(':') {
            let (condition, next) = s.split_once(':').unwrap();
            let condition = Condition::parse(condition);
            Rule {
                condition: Some(condition),
                name: next.to_string(),
            }
        } else {
            Rule {
                condition: None,
                name: s.to_string(),
            }
        }
    }

    fn apply(&self, range: PartRange) -> (Option<(PartRange, String)>, Option<PartRange>) {
        match &self.condition {
            Some(condition) => {
                let (range, next) = condition.apply(range);
                (range.map(|r| (r, self.name.clone())), next)
            }
            None => (Some((range, self.name.clone())), None),
        }
    }
}

#[derive(Debug)]
struct Condition {
    category: char,
    operation: char,
    value: usize,
}

impl Condition {
    fn parse(s: &str) -> Self {
        let mut chars = s.chars();
        Condition {
            category: chars.next().unwrap(),
            operation: chars.next().unwrap(),
            value: s[2..].parse().unwrap(),
        }
    }

    fn apply(&self, range: PartRange) -> (Option<PartRange>, Option<PartRange>) {
        match self.operation {
            '<' => range.split(self.category, self.value),
            '>' => {
                let (left, right) = range.split(self.category, self.value + 1);
                (right, left)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartRange {
    fn split(self, category: char, value: usize) -> (Option<PartRange>, Option<PartRange>) {
        match category {
            'x' => {
                if value < self.x.0 {
                    (None, Some(self))
                } else if value > self.x.1 {
                    (Some(self), None)
                } else {
                    (
                        Some(PartRange {
                            x: (self.x.0, value - 1),
                            m: self.m,
                            a: self.a,
                            s: self.s,
                        }),
                        Some(PartRange {
                            x: (value, self.x.1),
                            m: self.m,
                            a: self.a,
                            s: self.s,
                        }),
                    )
                }
            }
            'm' => {
                if value < self.m.0 {
                    (None, Some(self))
                } else if value > self.m.1 {
                    (Some(self), None)
                } else {
                    (
                        Some(PartRange {
                            x: self.x,
                            m: (self.m.0, value - 1),
                            a: self.a,
                            s: self.s,
                        }),
                        Some(PartRange {
                            x: self.x,
                            m: (value, self.m.1),
                            a: self.a,
                            s: self.s,
                        }),
                    )
                }
            }
            'a' => (
                Some(PartRange {
                    x: self.x,
                    m: self.m,
                    a: (self.a.0, value - 1),
                    s: self.s,
                }),
                Some(PartRange {
                    x: self.x,
                    m: self.m,
                    a: (value, self.a.1),
                    s: self.s,
                }),
            ),
            's' => (
                Some(PartRange {
                    x: self.x,
                    m: self.m,
                    a: self.a,
                    s: (self.s.0, value - 1),
                }),
                Some(PartRange {
                    x: self.x,
                    m: self.m,
                    a: self.a,
                    s: (value, self.s.1),
                }),
            ),
            _ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}
