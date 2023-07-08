fn main() {
    let three = sum_of_multiples_till(3, 999);
    let five = sum_of_multiples_till(5, 999);
    let fifteen = sum_of_multiples_till(15, 999);

    println!("Solution for 001 -> {:?}", three + five - fifteen)
}

fn sum_of_multiples_till(num: u32, limit: u32) -> u32 {
    let last_multiple = limit - (limit % num);
    ((num + last_multiple) * (last_multiple / num)) / 2
}

#[cfg(test)]
mod tests {
    use crate::sum_of_multiples_till;

    #[test]
    fn basic_test() {
        let three = sum_of_multiples_till(3, 9);
        let five = sum_of_multiples_till(5, 9);
        let fifteen = sum_of_multiples_till(15, 9);

        assert_eq!(23, three + five - fifteen);
    }
}
