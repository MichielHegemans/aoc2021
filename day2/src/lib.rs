use std::str::FromStr;

pub enum Position {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();

        if split.len() != 2 {
            return Err(());
        }

        let value: i64 = split[1].parse().map_err(|_| ())?;

        match split[0] {
            "forward" => Ok(Position::Forward(value)),
            "up" => Ok(Position::Up(value)),
            "down" => Ok(Position::Down(value)),
            _ => Err(())
        }
    }
}

#[derive(Default)]
struct Navigation {
    horizontal: i64,
    depth: i64,
    aim: i64,
}

impl Navigation {
    fn forward(self, x: i64) -> Self {
        Self {
            horizontal: self.horizontal + x,
            depth: self.depth + self.aim * x,
            aim: self.aim,
        }
    }

    fn up(self, x: i64) -> Self {
        Self {
            horizontal: self.horizontal,
            depth: self.depth,
            aim: self.aim - x,
        }
    }

    fn down(self, x: i64) -> Self {
        Self {
            horizontal: self.horizontal,
            depth: self.depth,
            aim: self.aim + x,
        }
    }
}

pub fn determine_position(input: &[Position]) -> i64 {
    let (horizontal, vertical) = input.iter().fold((0, 0), |acc, item| {
        match item {
            Position::Forward(x) => (acc.0 + x, acc.1),
            Position::Up(x) => (acc.0, acc.1 - x),
            Position::Down(x) => (acc.0, acc.1 + x),
        }
    });

    horizontal * vertical
}

pub fn determine_aimed_position(input: &[Position]) -> i64 {
    let nav = input.iter().fold(Navigation::default(), |acc, item| {
        match item {
            Position::Forward(x) => acc.forward(*x),
            Position::Up(x) => acc.up(*x),
            Position::Down(x) => acc.down(*x),
        }
    });

    nav.horizontal * nav.depth
}