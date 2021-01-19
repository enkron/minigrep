use std::env;

pub fn say_hello() {
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hello there. {}", n),
        None => panic!("Didn't recieve any name"),
    }
}
