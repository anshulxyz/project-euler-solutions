#[allow(dead_code)]
mod problems;

fn main() {
    println!("Hello, Project-Euler!");
    println!("p001 {}", problems::problem001::solve(1000));
    println!("p002 {}", problems::problem002::solve(4000000));
}
