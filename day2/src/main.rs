use day2::{determine_aimed_position, determine_position, Position};

static DATA: &str = include_str!("data.in");

fn main() {
    let values: Vec<Position> = DATA.lines().map(|x| x.parse().unwrap()).collect();

    let position = determine_position(&values);
    let aimed_position = determine_aimed_position(&values);

    println!("Pos: {}", position);
    println!("Aim: {}", aimed_position);
}