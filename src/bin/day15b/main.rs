use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day15.txt").unwrap();
    let reader = io::BufReader::new(file);

    let result: usize = reader
        .split(b',')
        .map(|s| s.unwrap())
        .map(|mut s| match s.pop() {
            Some(b'-') => (s, None),
            Some(focal) => {
                s.pop();
                (s, Some(focal - b'0'))
            }
            None => unreachable!(),
        })
        .fold(vec![vec![]; 256], |mut boxes, (label, focal)| {
            let b: &mut Vec<(Vec<u8>, u8)> = &mut boxes[hash(&label)];
            let i = b.iter().position(|(l, _)| l == &label);

            if let Some(focal) = focal {
                if let Some(i) = i {
                    b[i].1 = focal;
                } else {
                    b.push((label, focal));
                }
            } else if let Some(i) = i {
                b.remove(i);
            }

            boxes
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(|(j, (_, focal))| (i + 1) * (j + 1) * focal as usize)
                .sum::<usize>()
        })
        .sum();

    println!("{result}");
}

fn hash(s: &[u8]) -> usize {
    s.iter().fold(0, |res, &x| (res + x as usize) * 17 % 256)
}
