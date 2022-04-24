mod enums;
mod error_handling;
mod generics;
mod lifetime;
mod ownership;
mod stack_heap;
mod structs;
mod traits;
mod unit_test;
mod vars;

fn main() {
    println!("Hello, world!");
    // vars::run();
    // stack_heap::run();
    // ownership::run();
    // generics::run();
    // lifetime::run();
    // structs::run();
    // enums::run();
    // traits::run();
    // error_handling::run();
    unit_test::run();
}
