trait Fruits {
    fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}
struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        15
    }
}

trait Summary {
    fn summarize(&self) -> String {
        // default
        String::from("Default")
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Default")
    }
}

struct Article {
    headline: String,
    location: String,
    auther: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} {} {}", self.headline, self.auther, self.location)
    }
}

impl Message for Article {
    fn message(&self) -> String {
        String::from("Article Message")
    }
}

struct Tweet {
    user_name: String,
    auther: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {} {}", self.user_name, self.auther, self.content)
    }
}

pub fn run() {
    let apple = Apple {};
    get_price(apple);

    let article = Article {
        headline: String::from("Head"),
        location: String::from("location"),
        auther: String::from("auther"),
        content: String::from("content"),
    };

    println!("{}", article.summarize());

    let tweet = Tweet {
        user_name: String::from("name"),
        auther: String::from("auther"),
        content: String::from("content"),
    };

    println!("{}", tweet.summarize());

    notify(&tweet);

    notify_another(&article);
}

fn get_price<T: Fruits>(fruit: T) {
    println!("price is {}", fruit.price());
}

fn notify(item: &impl Summary) {
    println!("Notify {}", item.summarize());
}

fn notify_another(item: &(impl Summary + Message)) {
    println!("sum {}", item.summarize());
    println!("message {}", item.message());
}
