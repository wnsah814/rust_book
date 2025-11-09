fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // this empties the String, making it equal to ""
    // clear --> mutable reference, but word is an immutable reference --> compile-time error

    println!("the first word is: {word}");

    let _s = "Hello, world!"; // _s is a &str, immutable reference

    let my_string = String::from("hello world");

    // everything below works the same way
    let _word = first_word_str(&my_string[0..6]);
    let _word = first_word_str(&my_string[..]);
    let _word = first_word_str(&my_string);

    let my_string_literal = "hello world";
    let _word = first_word_str(&my_string_literal[0..6]);
    let _word = first_word_str(&my_string_literal[..]);
    let _word = first_word_str(my_string_literal);

    // Slices of arrays (will talk detail on Chapter 8)
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}