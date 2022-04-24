enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    println!("Stack Start");

    // 7MB
    // let a1: [u8; 7000000] = [1; 7000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 8];

    println!("{:p}", &v1);
    println!("{:?}", v1.as_ptr());
    println!("length is {}", v1.len());

    v1.insert(1, 10);

    v1.remove(0);
    println!("{:?}", v1);

    v1.append(&mut v3);
    println!("{:?}", v1);

    let t1: (i64, String) = (10, String::from("hello"));

    println!("{:p}", &t1);
    println!("{:?}", t1.1.as_ptr());

    let mut b1 = Box::new(t1);
    (*b1).1 += "add";

    println!("{:?}", b1);
    println!("{:p}", &b1);
}
