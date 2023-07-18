fn main() {
    println!("Solution for 002 -> {:?}", fib_even_sum(4000000));
}

fn fib_even_sum(limit: u32) -> u32 {
    let mut first: u32 = 1;
    let mut second: u32 = 1;

    let mut next = first + second;
    let mut sum = 0;

    while next < limit {
        if next % 2 == 0 {
            sum += next;
        }
        first = second;
        second = next;
        next = first + second;
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::fib_even_sum;

    #[test]
    fn it_works() {
        assert_eq!(fib_even_sum(10), 10);
        assert_eq!(fib_even_sum(100), 44);
    }
}
