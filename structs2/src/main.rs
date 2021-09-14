// Ownership of Struct Data - Won't compile - lifetime is required (Rust Chapter 10)
// Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: "bigbangtheory11@example.com",
        username: "bigbang",
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

    user1.username = "Dwayne Rainford";

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

    println!(
        "Inactive User1 Info - active: {}, username: {}",
        inactive_user1.active, inactive_user1.username
    );
}
