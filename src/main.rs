use crate::fibonacci::fibonacci_dynamic;
#[allow(unused_imports, dead_code)]
mod csv;
//mod driver;
mod example;
mod fibonacci;
mod pizza_ordering;

fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let num = 12;
    let result = factorial(num);
    println!("Factorial of {} is: {}", num, result);

    let fib_num = fibonacci_dynamic(num);
    println!("fibonacci of {} is: {}", num, fib_num);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_name() {
        let _name = String::from("william");

        println!("my first test");
    }
}
