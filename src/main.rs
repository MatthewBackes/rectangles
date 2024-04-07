#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self { // Associated Function
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 80,
        height: 65,
    };

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The width of rect1 is non-zero and is equal to {}", rect1.width);
    }

    println!("Can rect1 contain rect2? ANSWER: {}", rect1.can_hold(&rect2));
    println!("Can rect1 contain rect3? ANSWER: {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(40);
    println!("Square made with associated function: {} {}", sq.height, sq.width);
}