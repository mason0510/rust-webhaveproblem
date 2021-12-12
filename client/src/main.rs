use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    // 1. tcp连接3000端口, 返回tcpstream对象, 代表一个tcp连接
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    // 2. 发送消息, 将字符串转为u8字节再写入tcp流
    stream.write("Hello, ".as_bytes()).unwrap();
    stream.write("I am client".as_bytes()).unwrap();

    // 3. 接收消息, 读取u8字节转为字符串输出
    let mut ret_array = [0; 17];
    stream.read(&mut ret_array).unwrap();

    println!(
        "Response of server: {:?}",
        str::from_utf8(&ret_array).unwrap()
    );

    // 4. tcpstream对象走出作用域, 连接断开
}
