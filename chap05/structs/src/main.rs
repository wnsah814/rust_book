// default structure
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structure
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structure
struct AlwaysEqual;

fn main() {
    // entire instance must be mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("anotheremail@example.com"),
        String::from("anotherusername456"),
    );

    // can no longer use user2 after this point
    // becuase the String in the username field of user2 is moved into user3
    // If we had given user3 new String values for both email and username,
    // then we could still use user2 after this point.
    let _user3 = User {
        email: String::from("sjmskm@gmail.com"),
        ..user2 // struct update syntax
    };

    // tuple struct instances
    let _black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    // destructuring a tuple struct
    // Require you to name the type of struct
    let Point(x, y, z) = origin; 
    println!("x: {x}, y: {y}, z: {z}");

    // unit-like struct instance
    // Chapter 10 will explain more about this type of struct
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // field init shorthand
        username, // field init shorthand
        active: true,
        sign_in_count: 1,
    }
}