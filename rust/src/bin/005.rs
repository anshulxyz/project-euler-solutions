fn main() {
    println!("{}", solve(20));
}

// brute force approach of finding lowest common multiple of numbers from 1 to n
fn solve(n: u32) -> u32 {
    let mut result = 0;
    for i in n..=u32::MAX {
        // we start with the assumption that the current number i is divisible
        // by all the number from 0 to n
        let mut divisible = true;

        for j in 1..=n {
            if i%j!=0 {
                // if we're proven wrong about our assumption
                divisible = false;
                break;
            }
        }

        // if the assumption still holds true, then we've found our number
        if divisible {
            result = i;
            // and therefore we can stop the further iterations
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_0() {
        assert_eq!(solve(10), 2520);
    }
}