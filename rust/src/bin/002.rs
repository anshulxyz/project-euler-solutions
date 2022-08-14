// https://projecteuler.net/problem=2

fn main() {
    println!("{}", solve(4000000));
}

fn solve(limit: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut num = 0;
    let mut sum = 0;
    while num < limit {
        num = a + b;
        if num % 2 == 0 {
            sum += num;
        }
        a = b;
        b = num;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 10);
        assert_eq!(solve(4000000), 4613732);
    }
}
