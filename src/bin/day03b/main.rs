use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day03.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut result = 0;

    let mut prev = lines.next().unwrap().unwrap();
    let mut current = lines.next().unwrap().unwrap();

    result += process("", &prev, &current);

    for line in lines {
        if let Ok(next) = line {
            result += process(&prev, &current, &next);
            prev = current;
            current = next;
        }
    }

    result += process(&prev, &current, "");

    println!("{result}");
}

fn process(prev: &str, current: &str, next: &str) -> i32 {
    let mut result = 0;

    let chars: Vec<char> = current.chars().collect();
    for i in chars
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '*')
        .map(|(i, _)| i)
    {
        let mut nums: Vec<i32> = vec![];

        if i > 0 && chars[i - 1].is_digit(10) {
            let mut j = i - 1;
            while j > 0 && chars[j - 1].is_digit(10) {
                j -= 1;
            }
            nums.push(current[j..i].parse().unwrap());
        }

        if i < chars.len() - 1 && chars[i + 1].is_digit(10) {
            let mut j = i + 1;
            while j < chars.len() - 1 && chars[j + 1].is_digit(10) {
                j += 1;
            }
            nums.push(current[i + 1..j + 1].parse().unwrap());
        }

        nums.append(&mut find(prev, i));
        nums.append(&mut find(next, i));

        if nums.len() == 2 {
            result += nums[0] * nums[1];
        }
    }

    result
}

fn find(s: &str, index: usize) -> Vec<i32> {
    if s.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();

    let chars: Vec<char> = s.chars().collect();

    if chars[index].is_digit(10) {
        let mut i = index;
        while i > 0 && chars[i - 1].is_digit(10) {
            i -= 1;
        }
        let mut j = index + 1;
        while j < chars.len() && chars[j].is_digit(10) {
            j += 1;
        }

        result.push(s[i..j].parse().unwrap());
        return result;
    }

    if index > 0 && chars[index - 1].is_digit(10) {
        let mut i = index - 1;
        while i > 0 && chars[i - 1].is_digit(10) {
            i -= 1;
        }
        result.push(s[i..index].parse().unwrap());
    }

    if index < chars.len() - 1 && chars[index + 1].is_digit(10) {
        let mut i = index + 2;
        while i < chars.len() && chars[i].is_digit(10) {
            i += 1;
        }
        result.push(s[index + 1..i].parse().unwrap());
    }

    result
}
