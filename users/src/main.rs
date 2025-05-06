#[derive(Debug)] // Add this line to derive the Debug trait
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    println!("Hello, world!");
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User1: {:?}", user1);
    let user2 = User {
        username: String::from("user2"),
        ..user1
    };
    println!("User2: {:?}", user2);
    println!("User1: {:?}", user1);
}
