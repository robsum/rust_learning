struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    println!("{} {} {} {}", user1.email, user1.username, user1.sign_in_count, user1.active);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com");

    println!("{} {} {} {}", user1.email, user1.username, user1.sign_in_count, user1.active);

    let user1 = build_user(String::from("email: String"), String::from("username: String"));

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    
    println!("{} {} {} {}", user2.email, user2.username, user2.sign_in_count, user2.active);

    let black = Color(0, 0, 0);

    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}