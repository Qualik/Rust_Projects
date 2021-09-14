fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    let mut user1 = User {
        email: String::from("bigbangtheory11@example.com"),
        username: String::from("bigbang"),
        active: true,
        sign_in_count: 1,
    };

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    println!("User's email is: {}", user1.email);

    user1.username = String::from("Dwayne Rainford");

    println!("User's name is: {}", user1.username);

    let mut user2 = build_user(
        String::from("test@testingincanada.com"),
        String::from("testCanada"),
    );

    println!("User2 email: {}, username: {}", user2.email, user2.username);


    // Creating Instances from Other Instances with Struct Update Syntax
    let inactive_user1 = User {
        active: false,
        ..user1
    };

    println!("Inactive User1 Info - active: {}, username: {}", inactive_user1.active, inactive_user1.username);

}

// Tuple Structs without names fields to create different types

fn tuple_structs() {
    struct Color(i32, i32, i32); 
    struct Point(i32, i32, i32); 
    
    // Note these are different types even though
    // the fields within the struct have the same types
    // They are different because you have to define each structs own type

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let (a, b, c) = origin; 
    let (r, g, b) = black;
    
    // Gives the individual values
    // dot notation would be: (.0.1.2)

    // Note () unit-type
}
