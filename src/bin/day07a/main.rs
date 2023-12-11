use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day07.txt").unwrap();
    let reader = io::BufReader::new(file);

    let m = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let mut rows: Vec<Row> = reader
        .lines()
        .map(|line| Row::parse(&line.unwrap()))
        .collect();

    rows.sort_by(|a, b| match a.hand_type.cmp(&b.hand_type) {
        Ordering::Equal => {
            for (a, b) in a.hand.chars().zip(b.hand.chars()) {
                if a != b {
                    return m[&a].cmp(&m[&b]);
                }
            }
            unreachable!();
        }
        result => result,
    });

    let result: usize = rows
        .iter()
        .enumerate()
        .map(|(i, row)| (i + 1) * row.bid)
        .sum();

    println!("{result}");
}

struct Row {
    hand: String,
    hand_type: HandType,
    bid: usize,
}

impl Row {
    fn parse(s: &str) -> Self {
        let (hand, bid) = s.split_at(s.find(' ').unwrap());
        Self {
            hand: hand.to_string(),
            hand_type: HandType::parse(hand),
            bid: bid.trim().parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn parse(s: &str) -> Self {
        let mut counter: HashMap<char, usize> = HashMap::new();
        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }

        match counter.len() {
            1 => Self::FiveKind,
            2 => match counter.values().max().unwrap() {
                4 => Self::FourKind,
                3 => HandType::FullHouse,
                _ => unreachable!(),
            },
            3 => match counter.values().max().unwrap() {
                3 => HandType::ThreeKind,
                2 => Self::TwoPair,
                _ => unreachable!(),
            },
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}
