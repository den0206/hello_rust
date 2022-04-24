pub fn run() {
    let res1 = division_option(30.5, 15.3);
    match res1 {
        Some(x) => println!("{:.3}", x),
        None => println!("None"),
    }

    let res2 = division_result(4.0, 2.0);
    match res2 {
        Ok(x) => println!("{:.3}", x),
        Err(e) => println!("{}", e),
    }

    let res3 = sum(&[3, 4]);
    match res3 {
        Some(x) => println!("{:.3}", x),
        None => println!("None"),
    }
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("not Allow"))
    } else {
        Ok(x / y)
    }
}

fn sum(array: &[i32]) -> Option<i32> {
    let a0 = array.get(0)?;
    let a1 = array.get(1)?;
    let a2 = array.get(2)?;

    Some(a0 + a1 + a2)
}
