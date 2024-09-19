fn main(){
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
