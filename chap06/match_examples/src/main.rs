fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five = {five:?}, six = {six:?}, none = {none:?}");

    // catch-all pattern
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // catch-all pattern
    }
    
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // wildcard pattern
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // wildcard pattern doing nothing
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // If we didn't handle this case, we'd get a compile-time error (Matches are exhaustive)
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}