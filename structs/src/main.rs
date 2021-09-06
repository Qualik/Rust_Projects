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
}
