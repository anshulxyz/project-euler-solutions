// https://projecteuler.net/problem=1

pub fn solve(number: i32) -> i32 {
    let mut sum: i32 = 0;
    for n in 1..number {
        if (n % 3 == 0) || (n % 5 == 0) {
            sum += n;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 23);
        assert_eq!(solve(20), 78);
    }
}
