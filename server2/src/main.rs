fn main() {
    let string = String::from("127.0.0.1:8080");
    let string_slice = &string[10..14];

    dbg!(&string);
    dbg!(string_slice);
    // server.run();
}

struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Self {
            address
        }
    }
    //self = this in Java
    fn run(self) {

    }
}
