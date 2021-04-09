pub(crate) fn main() {
    // Repetition with Loops

    // It is often useful to execute a block of code more than once
    // For this Rust provides 'Loops'

    // A loop runs through the code inside the loop body to
    // the end and then starts immediately back at the beginning

    // Rust has three kinds of loops: 'loop', 'while' and 'for'

    // Repeating Code with 'loop'

    loop {
        println!("again!");

        // Returning Values from Loops

        // Pass the value of the result of a loop to the rest of the code by adding a 'break' expression
        // to stop the loop.

        // Declare a var named 'counter' and initialize with the value '0'

        let mut counter = 0;

        // Then declare the var 'result' to hold the value from the loop
        let result = loop {
            // Every iteration of the loop add 1 to the 'counter' var
            counter += 1;
            // Check whether the 'counter' is equal to 10
            if counter == 10 {
                // When it is use 'break' with the 'counter' value 2
                break counter * 2;
            }
        }; // After the loop add a semi-colon to end the statement that assigns the value to 'result'
           //   Print the value of 'result' (20)
        println!("The result is {}", result);

        // Conditional Loops with 'while'

        // Evaluates a condition within a loop
        // Also try and do this using 'loop', 'if', 'else' and 'break'.

        // Using 'while' eleminates the need to use the above keywords and is clearer.

        let mut number = 3;

        while number != 0 {
            println!("{}!", number);

            number -= 1;
        }
    }

    println!("LIFTOFF!!!!");

    // 'while' removes the nesting required by using 'loop', 'if', 'else' and 'break'.
    // It is also clearer so while a condition holds true, the code runs otherwise it exits the loop

    // Looping Through a Collection with 'for'
    // You can use the 'while' construct to loop over the elements in an 'array' as shown below.

    // counts through all of the elements in the 'Array'
    let a = [10, 20, 30, 40, 50];
    // 'index' starts at '0'
    let mut index = 0;
    // loop until the final 'index' in the 'Array' is no longer true: index < 5
    while index < 5 {
        // Prints the result - this will print every element in the 'Array'
        println!("the value is: {}", a[index]);
        // The loop will stop once the value of 5 is reached
        // iterates by 1 until 5 is reached
        index += 1;

        // The above approach is error prone as
        // the program can fail or panic if the
        // index length is incorrect.

        // A better way to do this is to use a 'for' loop
        // and execute some code for each item in an 'Array'

        // This will print every element in the 'Array'
        // just like the previous example but have increased code safety
        // removing the chance of bugs that might result from going
        // beyond the 'Array' or not going far enough and missing some items.

        let a = [10, 20, 30, 40, 50];

        for element in a.iter() {
            println!("the value is: {}", element);
        }

        // The below example causes the prgam to panic
        // The definition on'a' has been change from 5 to 4 elements
        let a = [10, 20, 30, 40];

        let mut index = 0;
        // But the conditions has not been updated to 'while index < 4;'
        while index < 5 {
            println!("the value is: {}", a[index]);

            index += 1;
            // This causes the program to panic.
            // Using the 'for' loop no other changes are required
            // if you have changed the number of values inside
            // an 'Array'.

            // 'Loops' are a commonly used construct in Rust.
            // In situations where you want to run some code a certain number of times
            // it is better to use a 'for' loop and a way to do this would be by using
            // a 'Range' which is provided by the 'std' standard library that
            // generates all numbers in sequence starting from one number and ending before another.

            // Here is an example of code for a countdown using a 'for' loop and another method
            // 'rev' to reverse the 'Range'.

            for number in (1..4).rev() {
                println!("{}!", number);
            }
            println!("LIFTOFF!!!");
        }
    }
}