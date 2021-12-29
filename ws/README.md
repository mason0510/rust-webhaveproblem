## 外部crate

- actix-web v3
- actix-rt v1.1.1

- serde, v1
- chrono, v0.4

## Actix 的并发

1. 异步I/O： 给定的OS原生线程在等待I/O时执行其他任务（例如监听网络连接）

2. 多线程并行： 默认情况下启动OS原生线程的数量与系统逻辑CPU数量相同

## 二进制包

cargo run -p webservice --bin server1

### `server1`

简单使用 crate: `actix-web` 和 `actix-rt` 建立一个简单的web服务器

- 文件

  - bin/server1.rs

- http路径
  - GET:  127.0.0.1：3000/health
    
    返回一个语句


### `teacher-service`

- 文件

  - bin/teacher-service.rs
  - handlers.rs
  - routers.rs
  - state.rs
  - models.rs

- http路径

  - GET:  /health
    返回`teacher-service`当前状态统计访问次数