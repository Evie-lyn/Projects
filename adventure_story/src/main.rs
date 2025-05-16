use std::io;

fn main() {
    println!("
It's late at night and the house is quiet except for a quiet grumple. 
It's your stomach... 
And the only thing that can silence it is Wingstop. 
You must retreive some delectable wings without alerting your parents!");

loop{
    println!("
Do you accept this mission? 
1 - Accept
2 - Decline");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    
    let answer: u8 = match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please type 1 or 2");
            continue;
        }
    };

    println!("You chose: {}", answer);

    if answer == 2 {
        println!("You ignore your stomach and go to bed hungry.
        
        GAME OVER
        ");
        std::process::exit(0);
    } else if answer == 1 {
        println!("Mission accepted!");
        break;
    } else {
        println! ("Invalid input. Please type 1 or 2");
        continue;
    }
}

loop{
    println!("
You decided to give in order Wingstop to be delivered. 
What flavor do you order?
1 - Atomic
2 - Hot Honey Rub
3 - Garlic Parmesan
4 - Plain");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    
    let answer: u8 = match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please type 1, 2, 3, or 4");
            continue;
        }
    };

    println!("You chose: {}", answer);

    if answer == 1 {
        println!("Atomic, Spicy!");
        break;
    } else if answer == 2 {
        println!("Hot Honey Rub, delicious!");
        break;
    } else if answer ==  3 {
        println! ("Garlic Parmesan, a classic!");
        break;
    } else if answer == 4 {
        println! (" Too boring, pick again.");
        continue;
    } else {
        println! ("Invalid input. Please type 1, 2, 3, or 4");
        continue;
    }
}

loop{
    println!("
You go to place your order, and it's asking for payment.
Do you:
1 - Steal your parents money and use it to pay
2 - Use your own money that you worked hard to get
3 - Decide you're not hungry anymore and cancel the order");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    
    let answer: u8 = match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please type 1 or 2");
            continue;
        }
    };

    println!("You chose: {}", answer);

    if answer == 1 {
        println!("Don't steal from your parents. 
        
        GAME OVER
        ");
        std::process::exit(0);
    } else if answer == 2 {
        println!("You use your own money.");
        break;
    } else if answer == 3 {
        println! ("You ignore your stomach and go to bed hungry.
        
        GAME OVER
        ");
        std::process::exit(0);
    } else {
        println! ("Invalid input. Please type 1 or 2");
        continue;
    }
}

loop{
    println!("
After an hour your wings have arrived! 
You sneak them down to your room and get ready to eat, when you hear a scratching at your door...
1 - Ignore it and dig in!
2 - Investigate the noise");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    
    let answer: u8 = match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please type 1 or 2");
            continue;
        }
    };

    println!("You chose: {}", answer);

    if answer == 1 {
        ignore_storyline();
        break;
    } else if answer == 2 {
        investigate_storyline();
        break;
    } else {
        println! ("Invalid input. Please type 1 or 2");
        continue;
    }
}

}

fn investigate_storyline(){
    println!("
You go to investigate the noise and you find your dog waiting outside your door.
He looks hungry...");

loop{
    println!("Do you 
1 - Let it in, and give your dog a fry
2 - Tell it to go to bed");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    
    let answer: u8 = match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please type 1 or 2");
            continue;
        }
    };

    println!("You chose: {}", answer);

    if answer == 2 {
        println!("Your dog snitches on you for not sharing.
        
        GAME OVER
        ");
        std::process::exit(0);
    } else if answer == 1 {
        println!("You share your meal with your dog, and it decides not to snitch on you.
You have avoided detection and gotten a delicious meal!
        
YOU WIN
");
        break;
    } else {
        println! ("Invalid input. Please type 1 or 2");
        continue;
    }
}

}

fn ignore_storyline(){
    println!("
Your dog, who was trying to get in, barks and snitches on you.

GAME OVER
");
std::process::exit(0);
}