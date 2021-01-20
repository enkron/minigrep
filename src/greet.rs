use std::env;

pub fn say_hello() {
    // The func parses command line arguments takes 2nd
    // (coz 1st it is a function name itself) and prints greetings
    // if there no arguments the func panics
    let name = env::args().skip(1).next();
    match name {
        Some(n) => println!("Hello there. {}", n),
        None => println!("Didn't recieve any name"),
    }
}
