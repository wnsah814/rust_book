fn main() {
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();

    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar"); // foobar

    let mut s = String::from("lo");
    s.push('l'); // lol

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    
    // note s1 has been moved here and can no longer be used
    let s3 = s1 + &s2; // Hello, world!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // note s1 has been moved here and can no longer be used
    let _s = s1 + "-" + &s2 + "-" + &s3; // tic-tac-toe

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // note s1, s2, and s3 can still be used here
    let s = format!("{}-{}-{}", s1, s2, s3); // tic-tac-toe

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}