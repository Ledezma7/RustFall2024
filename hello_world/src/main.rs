/*fn main(){
    const FREEZE_POINT_F: f64 = 32.0;

    let mut temp_f = FREEZE_POINT_F;

    let initial_temp_c = fahrenheit_to_celcius(temp_f);
    println!("{}F is {:.2}C", temp_f, initial_temp_c);

    for i in 1..=5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celcius(temp_f);
        println!("{}F is {:.2}C", temp_f, temp_c);

    }  
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
*/


/*fn main(){
    //create an array of numbers
    let nums: [i32; 10] = [7, 9, 11, 15, 21, 23, 28, 32, 2, 5];

    // for loop
    for &number in &nums {
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        }
        else if number % 3 == 0 {
            println!("Fizz");
        }
        else if number % 5 == 0 {
            println!("Buzz");
        }
        else {
            // check is number is even/odd
            if is_even(number) {
                println!("{} is even", number);
            }
            else {
                println!("{} is odd", number);
            }
        }
    }


    // use a while loop to sum up all numbers
    let mut sum = 0;
    let mut index = 0;
    while index < nums.len() {
        sum += nums[index];
        index += 1;
    }
    println!("The sum of all numbers: {}", sum);

    // use a loop to find largest number
    let mut largest = nums[0];
    for &number in &nums {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number is: {}", largest);

}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
*/

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