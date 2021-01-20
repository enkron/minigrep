mod greet;

fn main() {
    greet::say_hello();

    //testing closures
    let double_it = |x: u32| x * 2;
    let value = 5;
    let twice = double_it(value);
    println!("{} doubled is {}", value, twice);

    let multiple_value = |b: u32, c: u32| {
        let z = b + c;
        z * twice
    };
    let execute_closure = multiple_value(4, 7);
    println!("Result from closure: {}", execute_closure);
}
