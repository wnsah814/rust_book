fn main() {
    let s = String::from("hello");
    
    takes_ownership(s); // s is no longer valid here

    let x = 5;
    makes_copy(x); // x is still valid

    let _s1 = gives_ownership(); // s1 gets ownership

    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2); // s2 is moved, s3 gets ownership

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn takes_ownership(s: String) {
    println!("{s}");
} // drop is called

fn makes_copy(x: i32) {
    println!("{x}");
} // nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}