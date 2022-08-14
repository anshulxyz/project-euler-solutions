// https://projecteuler.net/problem=6

fn main() {
    println!("{}", solve(100));
}


fn solve(num: i32) -> i32 {
    square_of_sum(num) - sum_of_square(num)
}

fn square_of_sum(num: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..=num {
        sum += i
    }
    sum.pow(2)
}

fn sum_of_square(num: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..=num {
        sum += i.pow(2);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_of_sum() {
        assert_eq!(square_of_sum(10), 3025);
    }

    #[test]
    fn test_sum_of_square() {
        assert_eq!(sum_of_square(10), 385);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 2640);
        assert_eq!(solve(100), 25164150);
    }
}
