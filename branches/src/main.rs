fn main() {
    // let number = 6;
    // If Expressions

    // if number  < 5 {
        // println!("condition was true");

    // } else {
        // println!("condition was false");
    // }
    // if number != 0 {
        // println!("number was something other than zero");
    // }

    // if number % 4 == 0 {
        // println!("number is divisble by 4");
    // Checks each 'if' expression in turn and executes the first body for the condition that holds true.
    // } else if number % 3 == 0 { 
        // The first 'true' condition
        // println!("number is divisble by 3");

    // } else if number % 2 == 0 { 
        // Not run as first condition holds true.
        // println!("number is divisble by 2");

    // } else {
        // println!("number is not divisble by 4, 3, or 2"); 
        // Not run as first condition holds true.
    // }

    // Rust will ONLY execute the block for the first 'true' condition.
    // Once it has found this, it won't check the rest.    

    // Note - too many 'else if' expressions can clutter your code which may cause you to have to refactor
    // Use 'match' for this instead - explained later
    

    // Using 'if' in a 'let' Statement

    // REMEMBER!
    // 'if' is an expression and used on the RIGHT of a 'let' statement.

    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // first condition is evaluates as it meets the condition
    // println!("The value of number is: {}", number);

    // let condition = true;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {}", number);
    // Else block is generated as a 'number' but is a string - &str so doesn't compile.
    // Variables must have a single type.

}
