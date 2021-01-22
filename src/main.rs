mod greet;

use std::env;

fn main() {
    let name = env::args().skip(1).next();
    // The func env::args() parses command line arguments takes 2nd
    // coz 1st it is a function name itself

    let user = greet::Hi {
        name,
        greetings: String::from("Greetings you!"),
    };

    greet::say_hello(user);

    //testing closures
    //let double_it = |x: u32| x * 2;
    //let value = 5;
    //let twice = double_it(value);
    //println!("{} doubled is {}", value, twice);

    //let multiple_value = |b: u32, c: u32| {
    //    let z = b + c;
    //    z * twice
    //};
    //let execute_closure = multiple_value(4, 7);
    //println!("Result from closure: {}", execute_closure);
}
