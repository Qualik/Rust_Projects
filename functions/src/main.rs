// println!("Hello, world!");
// Functions
// another_function(5, 6);
// declaration of another_function
//  has one parameter named x. The type of x is specified as i32. When 5 is passed
//  to another_function the printlin! macro puts 5 where the curly brackets
//  were in the string format

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);

// let x = (let y = 6);

// let y = 6;
// the 6 in the statement is an expression that evaluates to the value 6.
// Also calling a function is an expression / calling a macro (println!) is an expression.

//     example -
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1 - Expressions do not include semi-colons - if you add a semicolon is becomes an statement
//     which then does not return a value.
//     };

//     println!("The value of y is: {}", y);
// }

// Function parameters
// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);

// Statements - Are instructions that perform some action and do not return a value.
//     Creating a var and assigning a value to it with the let keyword is a statement.
//     example -
//     fn main () {
//         let y = 6;
//     }

// Expressions - Evaluate to a resulting value.
fn main() {
    println!("Hello Rust!");

    another_function(117, 343);

    let y = -5;
    // println!("The value of y is: {}", y);

    let y_plus_three = add_three(y);
    // println!("The value of y_plus_three is: {}", y_plus_three);

    let number = -5;
    let is_positive = if number >= 0 { true } else { false };
    // println!("The value of is_positive is: {}", is_positive);

    // count_to_ten();

    // countdown_from(10);

    // countdown_to_five();

    countdown_from_five();
}
// divisible_by(25)
// println!(
//     "Is y_plus_three greater than five? Answer: {}",
//     greater_than_five(y_plus_three)
// );

fn another_function(x: u32, y: u32) {
    // arguement inside parentheses
    println!(
        "Here the value of x is: {} \n Here the value of y is: {}",
        x, y
    );
}

// expression_example();

fn expression_example() -> u32 {
    // Statement - does not return a value - add the ';' evaluates to a statment

    // Expression - evaluate to something - leave off the ';' evalutes to an expression

    let x = 3;
    x + 1 // this is an implicit return as no semicolon will evaluate as a statement
}
// Implicit Return
fn add_three(x: i32) -> i32 {
    x + 3
    // The semicolon is left off so that this
    // code will implicitly return the value / result.
}
// Control Flow - if / else expression
// fn greater_than_five(x: i32) -> bool {
//     if x > 5 {
//         true
//     } else {
//         false
//     }

// fn divisible_by(number: u32) {
//     match divisible_by {
//         number % 4 == 0 => println!("number is divisible by 4"),
//         number % 2 == 0 => println!("number is divisible by 2"),
//         number % 3 == 0 => println!("number is divisible by 3"),
//         _ =>  println!("number is not divisible by 4, 3, or 2"),

//     }

fn count_to_ten() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is: {}", result);
}

fn countdown_from(x: u32) {
    let mut number = x;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!!");
}

fn countdown_to_five() {
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("{}!", number);
    }
}

fn countdown_from_five() {
    for number in (1..6).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!!");
}
