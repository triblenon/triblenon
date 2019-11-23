#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq1 = Rectangle::square(11);

    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rect1 is {} square pixels.",
        rect1.area()
    );

    println!("rect2 is {:#?}", rect2);
    println!(
        "The area of the rect2 is {} square pixels.",
        rect2.area()
    );

    println!("rect3 is {:#?}", rect3);
    println!(
        "The area of the rect3 is {} square pixels.",
        rect3.area()
    );

    println!("sq1 is {:#?}", sq1);
    println!(
        "The area of the sq1 is {} square pixels.",
        sq1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Can rect1 hold sq1? {}", rect1.can_hold(&sq1));
    println!("Can rect2 hold sq1? {}", rect2.can_hold(&sq1));
    println!("Can rect3 hold sq1? {}", rect3.can_hold(&sq1));
}
