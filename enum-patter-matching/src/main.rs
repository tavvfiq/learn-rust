enum IpAddrKind {
    V4,
    V6,
}

// using struct to contain the kind and address
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// more concise to using enums
// enum IpAddrEnum{
//     V4(String),
//     V6(String),
// }

// more concise and can store multiple data with different type
enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum has 4 variants with different types
#[derive(Debug)]
enum Message {
    Quit,                       // no data associated with it
    Move { x: i32, y: i32 },    // has name fields like struct
    Write(String),              // includes a single value
    ChangeColor(i32, i32, i32), // include tuple
}

// enum can also have methods
impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let home_enum = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback_enum = IpAddrEnum::V6(String::from("::1"));
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);
    let m = Message::Write(String::from("Hello world"));
    m.call();
    // using option to presenting null value (similliar to "T?")
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // will error. because compiler always assume x is i8. therefore the y can be either None or Some(T)
    // pattern matching control flow
    // can use pattern matching to get the y value
    let sum = match y {
        None => x,
        Some(val) => x + val,
    };
    println!("{}", sum);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let my_cents = value_in_cents(Coin::Penny);

    // catch-all patterns and the _ placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other ), // all case other than 3 and 7 will enter this arm
        // _ => reroll() // can also use "_" to tell rust that we dont want to use the value
        _ => () // can also use expression unit value (empty tuple)
    }

    // concise control flow with "if let"
    // without if let
    let config_max: Option<u8> = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // with if let
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    if let None = config_max {
        println!("no maximum value");
    }
    // can also be used with else
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // matching with option
    // must cover both None and Some(T)
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// match pattern example
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // can also include block
            println!("Lucky penny!");
            return 1;
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn route(ip_kind: IpAddrKind) {}
