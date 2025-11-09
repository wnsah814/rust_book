fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {s1} is {len}");

    let mut s1 = s1; // make s1 mutable
    change(&mut s1);

    let mut s = String::from("hello");

    let r1 = &s; // whether mutable or not, you can have multiple immutable references
    let r2 = &s;
    // let r3 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
    // println!("{r1}, {r2}, and {r3}");

    println!("{r1} and {r2}");
    // r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem because no other references are active
    println!("{r3}");

    let _reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // borrowing
    s.len()
}

fn change(some_string: &mut String) { // mutable reference
    some_string.push_str(", world");
}

// dangling reference example (references must always be valid)
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s // error: returns a reference to data owned by the function
// }

fn dangle() -> String {
    let s = String::from("hello");

    s // return the String, transferring ownership
}