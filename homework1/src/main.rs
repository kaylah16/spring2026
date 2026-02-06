// ------- Temperature Converter ---------
fn assn1(){
    const FREEZING_POINT :f64 = 32.0; //constant of freezing point

    // conversions
    fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - FREEZING_POINT) * (5.0/9.0)
    }
    fn celsius_to_fahrenheit(c: f64) -> f64{
        (c * (9.0/5.0)) + FREEZING_POINT
    }

    let mut temperature: f64 = 32.0;
    println!("Starting temperature: {}F", temperature);
    //println!("fahrenheit: {}",fahrenheit_to_celsius(temperature));
    println!("celsius to fahrenheit: {}C", celsius_to_fahrenheit(temperature));

    let mut count = 6;
        //converts next 5 temperature values
    while count != 0 {
        count -= 1;
        println!("fahrenheit to celsius temp:{}F -> {}C", temperature, fahrenheit_to_celsius(temperature));
        temperature += 1.0;
        //println!("updated temp in fahrenheit: {}", temperature);
    }
}

// ------- Number Analyzer --------
fn assn2(){
    fn is_even(n: i32) -> bool {
        let even = n % 2;
        //returns true if number is even, false if odd
        if even == 0 {
            return true;
        }
        return false;
    }

    //array of 10 ints, random numbers
    let numbers: [i32; 10] = [2, 6, 18, 39, 10, 55, 86, 25, 105, 44];

    // for loop to print if number is even, fizz, buzz, or fizzbuzz
    for num in numbers{
        //determine if even or odd with function
        println!("{} is even?: {}", num, is_even(num));
    
        let by3 = num % 3;
        let by5 = num % 5;
        
        // divisble by both
        if 0 == by3 && 0 == by5{
            println!("{}: FizzBuzz", num);
        }
        // divisible by 3
        else if 0 == by3{
            println!("{}: Fizz", num);
        }
        //divisble by 5
        else if 0 == by5 {
            println!("{}: Buzz", num);
        }
    }
    let mut count = 0; //get index of array
    let mut total = 0; //get sum

    //let length = numbers.len();
    
    // while loop to get the sum of array
    while count < numbers.len() {
        // get sum of array
        //println!("{}", count); //check which index
       total += numbers[count];
       count += 1;
    }
    println!("Sum of array: {}", total);
    
    // for loop to find largest number
    let mut biggest = 0;
    for idx in 0..numbers.len(){
        // find largest int
        if biggest < numbers[idx]{
            biggest = numbers[idx];
        }
    }
    println!("Biggest integer is: {}", biggest);
    
} 

// ------ Guessing Game --------
fn assn3(){
    fn check_guess(guess: i32, secret: i32) -> i32 {
        //needs to be a match case?
        println!("The guessed number is: {}", guess);
        if guess == secret{
            return 0;
        }
        else if guess > secret{
            return 1;
        }
        else{
            return -1;
        }
    }

    // track gusses
    let mut num_guesses = 0;
    let mut guess_number = 28;
    let secret_num = 39;

    //loop
    loop {
        //call function
        let correct_guess = check_guess(guess_number, secret_num);

        if correct_guess == 0{
            println!("Guess is correct!");
            num_guesses += 1;
            break;
        }
        else if correct_guess == 1 {
            println!("Guess is too high!");
            num_guesses += 1;
            guess_number -= 1; //change value of number
        }
        else{
            println!("Guess is too low!");
            num_guesses += 1;
            guess_number += 3; //change value of number
        }
    }
    // print total gusses
    println!("Total guesses: {}", num_guesses);
}


fn main(){
    assn1();
    assn2();
    assn3();
}