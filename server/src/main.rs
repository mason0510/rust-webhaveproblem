use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Listening on port 3000 ...");

    // let result = listener.accept().unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("Connection established");

        let mut ret_arry = [0; 1024];

        stream.read(&mut ret_arry).unwrap();
        stream.write(&mut ret_arry).unwrap();
    }
}
