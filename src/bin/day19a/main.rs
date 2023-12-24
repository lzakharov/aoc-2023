use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day19.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    for line in &mut lines {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }

            let (name, rules) = line.split_at(line.find('{').unwrap());
            let rules = rules[1..rules.len() - 1]
                .split(',')
                .map(Rule::parse)
                .collect();
            workflows.insert(name.to_string(), Workflow { rules });
        }
    }

    let mut result = 0;

    for line in &mut lines {
        if let Ok(line) = line {
            let part = Part::parse(&line);

            let mut name = "in";

            while let Some(workflow) = workflows.get(name) {
                for rule in &workflow.rules {
                    if let Some(next) = rule.apply(&part) {
                        name = next;
                        break;
                    }
                }
            }

            if name == "A" {
                result += part.value();
            }
        }
    }

    println!("{result}");
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule(Option<Condition>, String);

impl Rule {
    fn parse(s: &str) -> Rule {
        if s.contains(':') {
            let mut chars = s.chars();
            let category = chars.next().unwrap();
            let operator = chars.next().unwrap();
            let (value, next) = s[2..].split_once(':').unwrap();
            let value = value.parse().unwrap();
            let next = next.to_string();
            let condition = Condition {
                category,
                operator,
                value,
            };
            Rule(Some(condition), next)
        } else {
            Rule(None, s.to_string())
        }
    }

    fn apply(&self, part: &Part) -> Option<&str> {
        if let Some(condition) = &self.0 {
            if !condition.check(part) {
                return None;
            }
        }
        Some(&self.1)
    }
}

#[derive(Debug)]
struct Condition {
    category: char,
    operator: char,
    value: usize,
}

impl Condition {
    fn check(&self, part: &Part) -> bool {
        match &self.operator {
            '>' => part.get(self.category) > self.value,
            '<' => part.get(self.category) < self.value,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(s: &str) -> Self {
        let nums: Vec<usize> = s[1..s.len() - 1]
            .split(',')
            .map(|x| x[2..].parse().unwrap())
            .collect();

        Part {
            x: nums[0],
            m: nums[1],
            a: nums[2],
            s: nums[3],
        }
    }

    fn get(&self, key: char) -> usize {
        match key {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}
