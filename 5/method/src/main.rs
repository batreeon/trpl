#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 多参数的方法。第二个参数是引用，表示这个方法并不需要获取参数的所有权。
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 这里的self和上面的self都代表的是 impl后面的 Rectangle
    fn square(size: u32) -> Self {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10, 
        height: 40,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 60,
    };
    println!("rect2 is{}in rect1, rect3 is{}in rect1",
        if rect1.can_hold(&rect2) {" "} else {" not "},
        if rect1.can_hold(&rect3) {" "} else {" not "}
    );

    // 参数里不带self的方法
    let rect4 = Rectangle::square(5);
    println!("rect4 {:#?}", rect4);
}