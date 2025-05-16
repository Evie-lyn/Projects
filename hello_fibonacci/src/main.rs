use std::io;

fn main() {
    println!("Fibonacci Sequence!");
    
    loop{
        println!("Please input the nth term.");
    
        let mut value = String::new();
    
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read line");
        
        let value: i64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer");
                continue;

            }
        };

        let suffix = match value.abs() %10 {
            1 if value.abs() % 100 != 11 => "st",
            2 if value.abs() % 100 != 12 => "nd",
            3 if value.abs() % 100 != 13 => "rd",
            _ => "th",
        };

        let fib_number = fibonacci(value);
        println!("The {}{} term is: {}", value, suffix, fib_number);
        break;
    }
}
fn fibonacci(n:i64) -> i64{
    match n{
        0 => 0,
        1 => 1,
        -1 => 1,
        _ if n > 1 => fibonacci(n-1) + fibonacci (n-2),
        _ if n < -1 => fibonacci(n+2) - fibonacci(n+1),
        _ => panic! ("Unexpected input value"),
    }
}