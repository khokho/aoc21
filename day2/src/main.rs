#[derive(Debug)]
enum Movement {
    Forward(isize),
    Up(isize),
    Down(isize),
}

impl Movement {
    fn new(line: &str) -> Self {
        let tokens: Vec<_> = line.split(' ').take(2).collect();
        let op = tokens[0];
        let n: isize = tokens[1].parse().unwrap();
        return match op {
            "forward" => Movement::Forward(n),
            "down" => Movement::Down(n),
            "up" => Movement::Up(n),
            _ => panic!("wrong op"),
        };
    }
}

#[derive(Default, Debug)]
struct Submarine {
    depth: isize,
    x: isize,
    aim: isize,
}

impl Submarine {
    // first star
    fn move_by_op(mut self, movement: Movement) -> Self {
        match movement {
            Movement::Forward(n) => self.x += n,
            Movement::Up(n) => self.depth -= n,
            Movement::Down(n) => self.depth += n,
        }
        self
    }

    fn move_by_new_op(mut self, movement: Movement) -> Self {
        match movement {
            Movement::Forward(n) => {
                self.x += n;
                self.depth += n * self.aim;
            }
            Movement::Up(n) => self.aim -= n,
            Movement::Down(n) => self.aim += n,
        }
        self
    }
}

fn main() {
    let input = include_str!("input").lines().filter(|l| l.len() > 0);
    let ops = input.map(Movement::new);
    let ship = Submarine::default();
    let ship = ops.fold(ship, Submarine::move_by_new_op);
    println!("result is {} this was done in rust", ship.x * ship.depth);
}
