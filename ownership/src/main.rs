fn main() {
    // let mut s = String::from("hello!");

    // s.push_str(", Rust!");
    // push_str() appends a string literal to a String   

    // println!("Our string is: {}", s);
    // This will print 'hello, Rust!'

    // Integers are simple values stored on the stack
    // so y is a copy of 5
    let x = 5;
    let y = x;

    let tup: (u32, bool) = (117, true);
    let tup_clone = tup;

    // Destructure tuple
    let (tup_x, _) = tup;
    let (tup_clone_x, _) = tup_clone;

    println!("x = {}, y = {}", x, y);
    println!("tup_x = {}, tup_clone_x = {}", tup_x, tup_clone_x);

    // can do things with s here as its valid

    // Example of a 'move'
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("s1 is: {}", s1);

    // Example of a 'clone'
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 is: {}, s2 is: {}", s1, s2);

}

// this scope now over and s is no longer valid
