struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn compare_arat(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.height * other.width
    }
}

pub fn run() {
    let rec1 = Rectangle {
        height: 30,
        width: 30,
    };

    let rec2 = Rectangle {
        height: 50,
        width: 50,
    };

    println!("{}", rec2.compare_arat(&rec1));
}

fn double_value(a: i32) -> i32 {
    a * 2
}

fn greeting(s: &str) -> String {
    format!("Hello {}", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_is_larger() {
        let rec1 = Rectangle {
            height: 30,
            width: 30,
        };

        let rec2 = Rectangle {
            height: 50,
            width: 50,
        };

        assert!(rec2.compare_arat(&rec1));
    }

    #[test]
    fn test_a_is_smaller() {
        let rec1 = Rectangle {
            height: 30,
            width: 30,
        };

        let rec2 = Rectangle {
            height: 50,
            width: 50,
        };

        assert!(!(rec1.compare_arat(&rec2)));
    }

    #[test]
    fn test_double() {
        let res = double_value(5);
        assert_eq!(10, res);
    }

    #[test]
    fn test_greet() {
        let res = greeting("RUST");
        assert!(res.contains("RUST"));
    }
}
