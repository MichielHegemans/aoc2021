pub fn count_increase(values: &[i64]) -> i64 {
    values
        .windows(2)
        .fold(0, |acc, item| {
            if item[0] < item[1] {
                return acc + 1;
            }

            return acc;
        })
}

pub fn count_sum(values: &[i64], window_size: usize) -> i64 {
    let values: Vec<i64> = values
        .windows(window_size)
        .map(|x| {
            x.iter().sum()
        })
        .collect();

    count_increase(&values)
}

#[test]
fn count_increase_increase_test() {
    let values = vec![0, 1];

    let increases = count_increase(&values);

    assert_eq!(increases, 1);
}

#[test]
fn count_increase_equal_test() {
    let values = vec![1, 1];

    let increases = count_increase(&values);

    assert_eq!(increases, 0);
}

#[test]
fn count_increase_decrease_test() {
    let values = vec![2, 1];

    let increases = count_increase(&values);

    assert_eq!(increases, 0);
}
