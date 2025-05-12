use std::io;
use std::collections::HashMap;

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

        let mut memo: HashMap<i64, i64> = HashMap::new(); //Creates memo table
        let fib_number = fibonacci(value, &mut memo);
        println!("The {}{} term is: {}", value, suffix, fib_number);
        break;
    }
}
fn fibonacci(n:i64, memo: &mut HashMap<i64, i64>) -> i64{ //check if n is in table
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    let result = match n {
        0 => 0,
        1 => 1,
        -1 => 1,
        _ if n > 1 => fibonacci(n-1, memo) + fibonacci (n-2, memo),
        _ if n < -1 => fibonacci(n+2, memo) - fibonacci(n+1, memo),
        _ => panic! ("Unexpected input value"),
    };

    memo.insert(n, result);
    result
}