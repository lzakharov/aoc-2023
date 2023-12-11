use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day03.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut result = 0;

    let mut prev = lines.next().unwrap().unwrap();
    let mut current = lines.next().unwrap().unwrap();

    result += sum("", &prev, &current);

    for line in lines {
        if let Ok(next) = line {
            result += sum(&prev, &current, &next);
            prev = current;
            current = next;
        }
    }

    result += sum(&prev, &current, "");

    println!("{result}");
}

fn sum(prev: &str, current: &str, next: &str) -> i32 {
    let mut result = 0;

    let chars: Vec<char> = current.chars().collect();
    for (left, right) in find(&current) {
        if (left > 0 && chars[left - 1] != '.')
            || (right < chars.len() && chars[right] != '.')
            || (!prev.is_empty() && check(&prev, left, right))
            || (!next.is_empty() && check(&next, left, right))
        {
            result += current[left..right].parse::<i32>().unwrap();
        }
    }

    result
}

fn find(s: &str) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut index = 0;
    let mut flag = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_digit(10) {
            if !flag {
                flag = true;
                index = i;
            }
        } else if flag {
            flag = false;
            result.push((index, i))
        }
    }

    if flag {
        result.push((index, s.len()));
    }

    return result;
}

fn check(s: &str, left: usize, right: usize) -> bool {
    let left = left - ((left > 0) as usize);
    let right = right + ((right < s.len() - 1) as usize);
    !s[left..right].chars().all(|c| c.is_digit(10) || c == '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(find("467..114.."), vec![(0, 3), (5, 8)]);
        assert_eq!(find("...*......"), vec![]);
        assert_eq!(find("..35..633.."), vec![(2, 4), (6, 9)]);
        assert_eq!(find(".664.598.."), vec![(1, 4), (5, 8)]);
    }
}
