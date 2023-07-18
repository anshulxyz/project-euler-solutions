fn main(){
    println!("{:?}", square_of_sums(100) - sum_of_squares(100));
}

fn sum_of_squares(n: u32) -> u32 {
    (n*(n+1)*(2*n + 1))/6
}

fn square_of_sums(n: u32) -> u32 {
    ((n*(n+1))/2).pow(2)
}

#[test]
fn test_006() {
    assert_eq!(385, sum_of_squares(10));
    assert_eq!(3025, square_of_sums(10));
}

