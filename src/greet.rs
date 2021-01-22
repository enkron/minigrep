pub struct Hi {
    pub name: Option<String>,
    pub greetings: String,
}

pub fn say_hello(hi: Hi) {
    match hi.name {
        Some(n) => println!("{} {}", hi.greetings, n),
        None => println!("Didn't recieve any name"),
    }
}
