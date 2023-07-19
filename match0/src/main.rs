#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucy penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum IpAddr {
    Ipv4,
    IPv6
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}


fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        },
        _ => println!("West"),
    }
    
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    }

    println!("{}", value_in_cents(Coin::Penny));

    let ip1 = IpAddr::IPv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);


    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            },
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            },
            Action::ChangeColorRGB(r, g, _) => {
                println!("change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g);

            }
        }
    }

    // If let patter - for match only-one match
    // Similar, while let ...
    let v = Some(3_u8);
    if let Some(3) = v {
        println!("three");
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // match here
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 1;
    match x {
        1 | 2 => println!("One or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Macro match!
    enum MyEnum {
        Foo,
        Bar
    }

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // error:
    // v.iter().filter(|x| x == MyEnum::Foo);
    v.iter().filter(|x| matches!(x, MyEnum::Foo));

    // .. 忽略全部剩余值
    // _ 忽略部分值

    // @ binding
    enum Message {
        Hello {id: i32}
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }

    match 1 {
        num @ (1 | 2) => {
            println!("{}", num);
        },
        _ => {}
    }
}
