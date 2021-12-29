## 包说明 packages

### client

1. tcp连接3000端口, 返回tcpstream对象, 代表一个tcp连接
2. 发送消息, 将字符串转为u8字节再写入tcp流
3. 接收消息, 读取u8字节转为字符串输出
4. tcpstream对象走出作用域, 连接断开

> 没有处理 io::Result<TcpStream> 的错误, 全部使用了 unwrap()

### echoserver

1. 取得绑定3000端口的tcplistener
2. 反复取得tcp返回的result对象, 每个result对象代表一个tcp连接
   1. 从result取得tcpstream对象
   2. 将收到的字节流原封不动返回回去
   3. tcpstream对象走出作用域, 连接断开

> 没有处理 io::Result<TcpStream> 的错误, 全部使用了 unwrap()

### http

- 定义了http请求体的数据结构， 实现了从http请求字符串解析成请求对象
- 定义了http响应体的数据结构， 实现了从响应体对象解析成字符串(字节流)

[详细文档](http/README.md)

### httpserver

- 定义了3000端口的监听 server.rs
- 请求路径的解析 router.rs
- 各个路径的处理 handler.rs
   