use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day02.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut result = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let (_, sets) = line.split_at(line.find(':').unwrap());

            let mut red_max = 0;
            let mut blue_max = 0;
            let mut green_max = 0;

            for set in sets[2..].split(';') {
                let mut red = 0;
                let mut blue = 0;
                let mut green = 0;

                for subset in set.trim().split(", ") {
                    let (n, color) = subset.split_at(subset.find(' ').unwrap());
                    let n = n.parse::<i32>().unwrap();
                    match &color[1..] {
                        "red" => red += n,
                        "blue" => blue += n,
                        "green" => green += n,
                        _ => unreachable!(),
                    }
                }

                red_max = red_max.max(red);
                blue_max = blue_max.max(blue);
                green_max = green_max.max(green);
            }

            result += red_max * blue_max * green_max;
        }
    }

    println!("{result}");
}
