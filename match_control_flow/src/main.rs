enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    NewYork,
    // --snip--
}

fn main() {
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State is : {:#?} .", state);
                25
            }
        }
    }

    println!("1st Coin is : {} .", value_in_cents(Coin::Penny));
    println!("2nd Coin is : {} .", value_in_cents(Coin::Nickel));
    println!("3rd Coin is : {} .", value_in_cents(Coin::Dime));
    println!("4th Coin is : {} .", value_in_cents(Coin::Quarter(UsState::NewYork)));

    println!();
    println!();

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let six = plus_one(Some(5));
    let none = plus_one(None);
    println!("Option<i32> : {:#?} .", six);
    println!("Option<i32> : {:#?} .", none);

    println!();
    println!();

    let some_u8_value = 5u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (println!("underscore")),
    }

    let three_u8_value = Some(3u8);
    if let Some(3) = three_u8_value {
        println!("three");
    } else {
        println!("one");
    }
}