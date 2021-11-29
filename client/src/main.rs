use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    stream.write("Hello, ".as_bytes()).unwrap();
    stream.write("I am client".as_bytes()).unwrap();
    let mut ret_array = [0; 17];
    stream.read(&mut ret_array).unwrap();

    println!(
        "Response of server: {:?}",
        str::from_utf8(&ret_array).unwrap()
    );
}
