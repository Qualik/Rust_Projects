fn main(){
    let s = String::from("hello"); // s comes into scope

    let s_taken_back = takes_ownership(s); // s_taken_back value moves its return value
    // into takes_ownership

    let x = 5; // x comes into scope

    // x would move into the function
    makes_copy(x); 
    // but i32 is Copy (meaning trait Copy), so it's ok to still 
    // use x afterwards 

    println!("x is now: {}", x);

    println!("s_taken_back is now: {}", s_taken_back);
}   // Able to use s_taken_back because take_ownership  returns it back

  // some_string comes into scope
fn takes_ownership(some_string: String) -> String {
    //will return a String
    println!("{}", some_string);
    // If some_string is added
    some_string
    // some_string is returned and moves out to the calling function.
    // no semicolon so implicitly returned - expression
} 

  // some_integer comes into scope
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens



// This function will move its return value into the
// function that calls it
fn gives_ownership() -> String {
    
    // some_string comes into scope
    // placing String onto the heap
    let some_string = String::from("hello");

    // some_string is returned and moves out to the calling function.
    some_string 
    // no semicolon so implicitly returned - expression
    // if semicolon added, it will NOT be returning a string
}

// will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string is returned and moves back out to the 
    // calling function.

    a_string
    // no semicolon so implicitly returned - expression

}