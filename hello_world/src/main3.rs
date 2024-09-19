fn main() {
    let number: i32 = 33;
    let guess: i32;
    let mut attempts: i32 = 0;

    
        guess = 33;
    loop {
        attempts += 1; 
        let result = check_guess(guess, number);

        if result == 0 {
            println!("Great Job! {} is the correct guess.", guess);
            break;
        } 
        else if result == 1 {
            println!("{} is too high. Try again.", guess);
        } 
        else if result == -1 {
            println!("{} is too low. Try again.", guess);
        }
    }
    println!("Amount of guesses: {}.", attempts);
}

fn check_guess(guess: i32, number: i32) -> i32 {
    if guess == number {
        0
    }
    else if guess > number {
        1
    }
    else {
        -1
    }
}