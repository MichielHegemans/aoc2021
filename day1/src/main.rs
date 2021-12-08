use day1::{count_increase, count_sum};

static DATA: &str = include_str!("data.in");

fn main() {
    let values: Vec<i64> = DATA.lines().map(|x| x.parse().unwrap()).collect();

    let increases = count_increase(&values);
    let sum_increase = count_sum(&values, 3);

    println!("Inc: {}", increases);
    println!("Sum: {}", sum_increase);
}
