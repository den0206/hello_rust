struct Point<T, U> {
    x: T,
    y: U,
}

pub fn run() {
    let number_list = vec![1, 2, 3, 4, 5];

    println!("{}", largist(number_list));

    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("{}", largist(char_list));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2 };
    println!("{}", p1.x);
}

fn largist<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largist = list[0];
    for i in list {
        if i > largist {
            largist = i;
        }
    }

    largist
}
