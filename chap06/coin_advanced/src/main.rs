// Enum representing US states
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    // ... other states
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::Arizona => year >= 1912,
            UsState::Arkansas => year >= 1836,
            UsState::California => year >= 1850,
            UsState::Colorado => year >= 1876,
            UsState::Connecticut => year >= 1788,
            UsState::Delaware => year >= 1787,
            UsState::Florida => year >= 1845,
            UsState::Georgia => year >= 1788,
            // ... other states with their statehood years
        }
    }
}

// Enum representing different types of coins
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),  // Quarter holds a UsState value
}

// Function to get the value of a coin in cents
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

// Function to describe a state quarter
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    
    // Example using if let
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
    
    // Example using describe_state_quarter function
    let coin2 = Coin::Quarter(UsState::Alabama);
    if let Some(description) = describe_state_quarter(coin2) {
        println!("{}", description);
    }
    
    // Example with config_max
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}