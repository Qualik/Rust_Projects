fn main(){
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function - takes_ownership
    //... and is no longer valid here

    let x = 5; // x comes into scope

    // x would move into the function
    makes_copy(x); 
    // but i32 is Copy (meaning trait Copy), so it's ok to still 
    // use x afterwards 

    println!("x is now: {}", x);

    println!("s is now: {}", s);
} // Here x goes ouit of scope, then s. But because s's value was moved, nothing
  // special happens

  // some_string comes into scope
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing
  // memory is freed

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