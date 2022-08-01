struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs without name fields
struct Color(i32, i32, i32); //rgb
struct Point(i32, i32, i32); //xyz

// unit-like struct
struct AlwaysEqual;

fn main() {
    // immutable
    let user1 = User {
        active: true,
        email: String::from("email@example.com"),
        username: String::from("john doe"),
        sign_in_count: 1,
    };
    // mutable
    let mut user2 = User {
        active: true,
        email: String::from("email@example.com"),
        username: String::from("john doe"),
        sign_in_count: 1,
    };
    user2.email = String::from("taufiq@mail.com");
    let user3 = build_new_user(String::from("email"), String::from("username"));
    // using update syntax
    let user4 = User {
        email: String::from("taufiq"),
        ..user3
    };
    let black = Color(0,0,0);
    // accessing with .{index} annotation
    println!("red: {}", black.0);
    let origin = Point(0,0,0);
    println!("x: {}", origin.0);
    let subject = AlwaysEqual;
    example_using_struct();
}

fn build_new_user(email: String, username: String) -> User {
    return User { active: true, username, email, sign_in_count: 1 }
}

fn example_using_struct() {
    let rect = (30, 50);
    println!("the area of rectangle is {} square pixels", area(rect));
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("the area of rectangle is {} square pixels", area_with_struct(&rect2));
    println!("rect is {:#?}", rect2);
    // calculate area using Rectangle's area method
    println!("the area of rectangle is {} square pixels", rect2.area());
    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }
    let rect3 = Rectangle {
        width: 100,
        height: 50,
    };
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    let square = Rectangle::create_square(20);
    println!("square is {:#?}", square);
}

fn area(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_with_struct(rect: &Rectangle) -> u32 {
    return rect.height * rect.width;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// method syntax. methods are similiar to function.
// but within the struct context.
// the first parameter always the self of the struct.
impl Rectangle {
    // can also written as "self: &Rectangle" or only "&self"
    fn area(self: &Self) -> u32 {
        return self.width *self.height;
    }
    // can also create a method with the same name as the field
    fn width(&self) -> bool {
        return self.width > 0;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
    // method can also dont have &self as the first parameters. (maybe called static method on c++?)
    // will use struct::method to call the methods
    // usually used to construct the struct
    fn create_square(size: u32) -> Rectangle {
        return Rectangle { width: size, height: size }
    }
}