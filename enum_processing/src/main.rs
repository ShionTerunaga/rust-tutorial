enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny = Coin::Penny;
    let select_coin = value_in_cents(penny);

    println!("First selection is {}", select_coin);

    let nickel = Coin::Nickel;
    let select_coin = value_in_cents(nickel);

    println!("Second selection is {}", select_coin);

    let dime = Coin::Dime;
    let select_coin = value_in_cents(dime);

    println!("Third selection is {}", select_coin);

    let quarter = Coin::Quarter;
    let select_coin = value_in_cents(quarter);

    println!("Fourth is selection is {}", select_coin);
}
