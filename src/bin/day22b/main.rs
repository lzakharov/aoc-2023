use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day22.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut x_max = 0;
    let mut y_max = 0;
    let mut z_max = 0;

    let mut bricks: Vec<Brick> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(&[',', '~'])
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|xs| Brick::new(&xs))
        .inspect(|brick| {
            x_max = x_max.max(brick.1.x);
            y_max = y_max.max(brick.1.y);
            z_max = z_max.max(brick.1.z);
        })
        .collect();

    bricks.sort_by(|a, b| a.0.z.cmp(&b.0.z));

    let mut space = vec![vec![vec![false; z_max + 1]; y_max + 1]; x_max + 1];

    for brick in &mut bricks {
        while brick.fall(&space) {
            brick.0.z -= 1;
            brick.1.z -= 1;
        }

        brick.fill(&mut space, true);
    }

    let mut result = 0;

    for (i, brick) in bricks.iter().enumerate() {
        bricks.iter().for_each(|brick| brick.fill(&mut space, true));
        brick.fill(&mut space, false);

        for (_, brick) in bricks.iter().enumerate().filter(|(j, _)| i != *j) {
            if brick.fall(&space) {
                brick.fill(&mut space, false);
                result += 1;
            }
        }
    }

    println!("{result}");
}

#[derive(Debug)]
struct Brick(Point, Point);

impl Brick {
    fn new(xs: &[usize]) -> Self {
        Brick(
            Point::new(xs[0].min(xs[3]), xs[1].min(xs[4]), xs[2].min(xs[5])),
            Point::new(xs[0].max(xs[3]), xs[1].max(xs[4]), xs[2].max(xs[5])),
        )
    }

    fn fill(&self, space: &mut [Vec<Vec<bool>>], value: bool) {
        for x in self.0.x..=self.1.x {
            for y in self.0.y..=self.1.y {
                for z in self.0.z..=self.1.z {
                    space[x][y][z] = value;
                }
            }
        }
    }

    fn fall(&self, space: &[Vec<Vec<bool>>]) -> bool {
        if self.0.z == 1 {
            return false;
        }

        if self.0.x == self.1.x && self.0.y == self.1.y {
            return !space[self.0.x][self.0.y][self.0.z - 1];
        }

        for x in self.0.x..=self.1.x {
            for y in self.0.y..=self.1.y {
                for z in self.0.z - 1..=self.1.z - 1 {
                    if space[x][y][z] {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Point { x, y, z }
    }
}
