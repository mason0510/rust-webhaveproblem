## 数据结构

### HTTP 请求

- 请求格式 HTTP request format

   | line         | content example         |
   | ------------ | ----------------------- |
   | Request Line | GET /greeting  HTTP/1.1 |
   | Header Line1 | Host:localhost:3000     |
   | Header Line2 | User-Agent:Curl/7.64.1  |
   | Header Line3 | Accept:*/*              |
   | Empty line   |                         |
   | Message body | [anything]              |

- 实现的trait

    | Trait      | 描述                                                        |
    | ---------- | ----------------------------------------------------------- |
    | From<&str> | 手动实现from方法, 用于把传进来的字符串切片转换为HttpRequest |
    | Debug      | #[derive()] 导入, 打印数据结构调试信息                      |
    | PartialEq  | #[derive()], 用于解析和自动化测试脚本做比较                 |

### HTTP 响应

- 响应格式 HTTP response format

    | line         | content example         |
    | ------------ | ----------------------- |
    | Status Line  | HTTP/1.1 200 ok         |
    | Header Line1 | Content-type：text/html |
    | Header Line2 | Content-Length 30       |
    | Empty line   |                         |
    | Message body | [anything]              |

- 实现的trait

  - Debug
  - PartialEq
  - Clone

  - 手动实现的trait

    | tarit           | 描述                              |
    | --------------- | --------------------------------- |
    | Default trait   | 指定成员的默认值                  |
    | new()           | 使用默认值创建一个对象            |
    | send_response() | 构建响应，将原始字节通过tcp传送   |
    | getter methods  | 数据成员的封装                    |
    | From<>          | 能将 `HttpResponse` 转为 `String` |

    