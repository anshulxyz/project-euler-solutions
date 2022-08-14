// https://projecteuler.net/problem=1

fn main() {
    println!("{}", solve(1000));
}

fn solve(number: i32) -> i32 {
    // referring Gauss' sum of arithmetic sequences
    let number_of_3s = (number - 1) / 3;
    let last_multiple_of_3 = 3 * number_of_3s;
    let sum_of_3s = ((last_multiple_of_3 + 3) * number_of_3s) / 2;

    // get number of 5s and last element divisible by 5
    let number_of_5s = (number - 1) / 5;
    let last_multiple_of_5 = 5 * number_of_5s;
    let sum_of_5s = ((last_multiple_of_5 + 5) * number_of_5s) / 2;

    // get number of 15s and last element divisible by 15
    let number_of_15s = (number - 1) / 15;
    let last_multiple_of_15 = 15 * number_of_15s;
    let sum_of_15s = ((last_multiple_of_15 + 15) * number_of_15s) / 2;

    let sum = (sum_of_3s + sum_of_5s) - sum_of_15s;

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 23);
        assert_eq!(solve(20), 78);
        assert_eq!(solve(1000), 233168);
    }
}
