pub mod sub_a;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    let mut _x: i32 = 5;
    _x = 6;

    println!("{}", _x);

    println!("{}", usize::BITS);
    println!("{:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;

    println!("{:p} {:p}", &i2, &i3);

    let y = 5;
    let y = y + 1;

    println!("{:p}", &y);

    let t1: (i32, f64, &str) = (500, 6.4, "str");

    println!("{} {} {}", t1.0, t1.1, t1.2);

    let mut t2: ((i32, i32), (i32, i32)) = ((0, 1), (0, 2));
    let ((ref mut x1_p, ref mut y1_p), _) = t2;
    *x1_p = 5;
    *y1_p = -5;
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];

    println!("{:?}, {:?} {} {}", a1, a2, a1[1], a1[4]);

    let s1: &str = "helloあ";

    println!("{:?}", s1.as_ptr());
    println!("{}", s1.len());
    println!("{}", s1.chars().count());

    let mut str1: String = String::from("String");
    let mut str2: String = String::from("Hello World");

    println!("{:p}", &str1);
    println!("{}", str1.capacity());

    str1.push_str("_New1");
    println!("{}", str1);
}
