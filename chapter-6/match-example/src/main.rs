#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 14,
        Coin::Nickel => 49,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            14
        }
        Coin::Dime => 9,
    }
}
fn main() {
    value_in_cents(Coin::Penny);
}
