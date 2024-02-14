fn main() {
    println!("value_in_cents: {}", value_in_cents(Coin::Dime));
    println!(
        "value_in_cents: {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    let five = Some(5);
    let six = plus_one1(five);
    let none = plus_one1(None);
    println!("{:?}, {:?}", six, none);

    // match 的每个分支的返回值要相同，或者都没返回值
    // o 匹配除1,2外的其他任意模式，并且用o来接收该值。注意要放在最后一项
    let x = 1;
    println!("{}", match x {
        1 => 1,
        2 => 2,
        o => o,
    });

    let x = 3;
    println!("{}", match x {
        1 => 1,
        2 => 2,
        o => o,
    });

    // _ 匹配除1,2外的其他任意模式，但不接收该值
    let x = 3;
    println!("{}", match x {
        1 => 1,
        2 => 2,
        _ => -1,
    });

    let x = 3;
    match x {
        1 => println!("1"),
        2 => println!("2"),
        _ => (),
    }

    // if let 让match更简洁
    let x = 1;
    if let 1 = x {
        println!("1");
    }else{
        ();
    }

    if let Coin::Quarter(state) = Coin::Quarter(UsState::Alabama) {
        println!("State quarter from {:?}!", state);
    }
}

#[derive(Debug)] // 这样可以立刻看到州的名称
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one1(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/* None not covered
fn plus_one2(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
*/