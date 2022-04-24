#[derive(Debug)]
struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let mut user1 = User {
        user_name: String::from("name"),
        email: String::from("email"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("email2");

    println!("{:#?}", user1);

    let user2 = build_user(String::from("email2"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(30, 30);

    println!("{:#?}", rect);
    rect.area();
}

fn build_user(email: String, username: String) -> User {
    let user = User {
        user_name: username,
        email,
        sign_in_count: 1,
        active: true,
    };

    user
}
