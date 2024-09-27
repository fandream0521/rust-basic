fn main() {
    let user = User {
        active: true,
        username: String::from("user"),
        email: String::from("user@qq.com"),
        sing_in_count: 1,
    };

    println!("user.username: {}", user.username);
    println!("user.email: {}", user.email);
    println!("user.sing_in_count: {}", user.sing_in_count);

    let user = build_user(String::from("someone@example.com"), String::from("someone"));
    println!("user.username: {}", user.username);
    println!("user.email: {}", user.email);
    println!("user.sing_in_count: {}", user.sing_in_count);

    let user = User {
        email: String::from("tom@example.com"),
        ..user
    };

    println!("user.username: {}", user.username);
    println!("user.email: {}", user.email);
    println!("user.sing_in_count: {}", user.sing_in_count);
    println!("user.active: {}", user.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);

    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_1(rect)
    );

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect: {:?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_2(&rect)
    );

    let rect1 = Rectangle {
        width: dbg!(30 + 10),
        height: dbg!(50),
    };
    println!("rect1: {:?}", rect1);
    dbg!(&rect1);
    println!("rect1: {:?}", rect1);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect2: {:?}", rect2);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect3 can hold rect4: {}", rect3.can_hold(&rect4));
    println!("rect3 can hold rect5: {}", rect3.can_hold(&rect5));

    let square = Rectangle::square(3);
    println!("square: {:?}", square);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sing_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sing_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
