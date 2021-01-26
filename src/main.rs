pub mod greet;

fn main() {
    let mut user = greet::User::generate("Rinzler", "male", "white", 67);
    println!("{:?}", user);

    user.set_age(113);
    println!("{:?}", user);
}
