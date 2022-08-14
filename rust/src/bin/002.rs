// https://projecteuler.net/problem=2

fn main() {
    println!("{}", solve(4000000));
}

fn solve(limit: i32) -> i32 {
    let mut a = 2;
    let mut b = 8;

    // refer https://projecteuler.net/overview=002
    let mut num = 4*b + a;
    let mut sum = 10;

    while num < limit {
        sum += num;
        a = b;
        b = num;
        num = 4*b + a;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_0() {
        assert_eq!(solve(10), 10);
    }

    #[test]
    fn test_solve_1() {
        assert_eq!(solve(4000000), 4613732);
    }
}
