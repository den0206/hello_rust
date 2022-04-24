pub fn run() {
    let s1 = String::from("Jello");
    let s2 = s1.clone();

    println!("{} {}", s1, s2);

    let i1 = 1;
    let i2 = i1;

    println!("{} {}", i1, &i2);

    let sl1 = "literal";
    let sl2 = sl1;

    let s5 = String::from("Hello");
    take_ownership(s5);

    let s6 = String::from("s7");
    let s7 = take_giveback_ownership(s6);
    let s8 = caluculate_length(&s7);

    println!("{} {}", s7, s8);

    let mut s9 = String::from("asdf");
    change(&mut s9);
    println!("{}", &s9);

    let mut s11 = String::from("s11");
    let r1 = &mut s11;

    let x = 5;
    let r = &x;

    println!("{}", r);

    println!("{}", x);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn caluculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_Add");
}
