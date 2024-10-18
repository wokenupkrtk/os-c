fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let number = 5;
    println!("The factorial of {} is {}", number, factorial(number));
}