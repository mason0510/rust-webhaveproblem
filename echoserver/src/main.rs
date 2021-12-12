use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 1. 取得绑定3000端口的tcplistener
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Listening on port 3000 ...");

    // 2. 反复取得tcp返回的result对象, 每个result对象代表一个tcp连接
    // let result = listener.accept().unwrap();
    for stream in listener.incoming() {
        // 1. 从result取得tcpstream对象
        let mut stream = stream.unwrap();
        println!("Connection established");

        // 2. 将收到的字节流原封不动返回回去
        let mut ret_arry = [0; 1024];
        stream.read(&mut ret_arry).unwrap();
        stream.write(&mut ret_arry).unwrap();

        // 3. tcpstream对象走出作用域, 连接断开
    }
}
