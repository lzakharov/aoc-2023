use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day05.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut ranges = (&lines.next().unwrap().unwrap()[7..])
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|x| Range::new(x[0], x[1]))
        .collect::<Vec<Range>>();

    let mut mem = vec![];

    for line in lines {
        if let Ok(line) = line {
            if line.is_empty() {
                ranges.append(&mut mem);
                continue;
            }

            if line.starts_with(|x: char| !x.is_digit(10)) {
                continue;
            }

            let row: Vec<i64> = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();

            let (dest, src, length) = (row[0], row[1], row[2]);

            let mut tmp = vec![];

            for i in 0..ranges.len() {
                let (left, right, intersect) = ranges[i].map(&Range::new(src, length), dest);
                if let Some(r) = left {
                    tmp.push(r);
                }
                if let Some(r) = right {
                    tmp.push(r);
                }
                if let Some(r) = intersect {
                    mem.push(r);
                }
            }

            ranges = tmp;
        }
    }

    ranges.append(&mut mem);

    println!("{}", ranges.iter().map(|x| x.start).min().unwrap());
}

#[derive(Debug, Clone, PartialEq)]
struct Range {
    start: i64,
    length: i64,
}

impl Range {
    fn new(start: i64, length: i64) -> Self {
        Self {
            start: start,
            length: length,
        }
    }

    fn end(&self) -> i64 {
        self.start + self.length - 1
    }

    fn map(&self, b: &Range, d: i64) -> (Option<Range>, Option<Range>, Option<Range>) {
        if self.start > b.end() || self.end() < b.start {
            return (Some(Range::new(self.start, self.length)), None, None);
        }

        if self.start >= b.start && self.end() <= b.end() {
            return (
                None,
                None,
                Some(Range::new(d + self.start - b.start, self.length)),
            );
        }
        if self.start >= b.start && self.start <= b.end() {
            return (
                Some(Range::new(b.end() + 1, self.end() - b.end())),
                None,
                Some(Range::new(
                    d + self.start - b.start,
                    b.end() - self.start + 1,
                )),
            );
        }
        if self.start < b.start && self.end() > b.end() {
            return (
                Some(Range::new(self.start, b.start - self.start)),
                Some(Range::new(b.end() + 1, self.end() - b.end())),
                Some(Range::new(d, b.length)),
            );
        }
        if self.start < b.start && self.end() <= b.end() {
            return (
                Some(Range::new(self.start, b.start - self.start)),
                None,
                Some(Range::new(d, self.end() - b.start + 1)),
            );
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        // 0 1 2 3 4 5 6 7 8 9
        // . [ . . ] . . . . .
        // . . . . . . ( . . )
        assert_eq!(
            Range::map(&Range::new(1, 4), &Range::new(6, 4), 128),
            (Some(Range::new(1, 4)), None, None),
        );

        // 0 1 2 3 4 5 6 7 8 9
        // . . . . . . [ . . ]
        // . ( . . ) . . . . .
        assert_eq!(
            Range::map(&Range::new(6, 4), &Range::new(1, 4), 128),
            (Some(Range::new(6, 4)), None, None),
        );

        // 0 1 2 3 4 5 6 7 8 9
        // . . . [ . . ] . . .
        // . . . ( . . ) . . .
        assert_eq!(
            Range::map(&Range::new(3, 4), &Range::new(3, 4), 128),
            (None, None, Some(Range::new(128, 4))),
        );

        // 0 1 2 3 4 5 6 7 8 9
        // . . . [ . . ] . . .
        // . ( . . . . . . . )
        assert_eq!(
            Range::map(&Range::new(3, 4), &Range::new(1, 9), 128),
            (None, None, Some(Range::new(130, 4))),
        );

        // 0 1 2 3 4 5 6 7 8 9
        // . . [ . . . ] . . .
        // . ( . . ) . . . . .
        assert_eq!(
            Range::map(&Range::new(2, 5), &Range::new(1, 4), 128),
            (Some(Range::new(5, 2)), None, Some(Range::new(129, 3))),
        );

        // 0 1 2 3 4 5 6 7 8 9
        // . . . . [ . . ] . .
        // . ( . . ) . . . . .
        assert_eq!(
            Range::map(&Range::new(4, 4), &Range::new(1, 4), 128),
            (Some(Range::new(5, 3)), None, Some(Range::new(131, 1))),
        );

        // 0 1 2 3 4 5 6 7 8 9
        // . [ . . . . . ] . .
        // . . . ( . ) . . . .
        assert_eq!(
            Range::map(&Range::new(1, 7), &Range::new(3, 3), 128),
            (
                Some(Range::new(1, 2)),
                Some(Range::new(6, 2)),
                Some(Range::new(128, 3))
            ),
        );

        // 0 1 2 3 4 5 6 7 8 9
        // . [ . . . . . ] . .
        // . . . ( . . . ) . .
        assert_eq!(
            Range::map(&Range::new(1, 7), &Range::new(3, 5), 128),
            (Some(Range::new(1, 2)), None, Some(Range::new(128, 5))),
        );

        // 0 1 2 3 4 5 6 7 8 9
        // . [ . . . . . ] . .
        // . . . . ( . . . . )
        assert_eq!(
            Range::map(&Range::new(1, 7), &Range::new(4, 6), 128),
            (Some(Range::new(1, 3)), None, Some(Range::new(128, 4))),
        );
    }
}
