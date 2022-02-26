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

### `server1` (示例)

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

  - POST: /courses/

    ```sh
    curl -H "Content-Type: application/json" -X POST -d '{"teacher_id":1, "name":"First course"}' "127.0.0.1:3000/courses/"
    ```

  - GET:  /courses/{teacher id}

    ```sh
    curl -H "Content-Type: application/json" -X POST -d '{"teacher_id":1, "name":"First course"}' "127.0.0.1:3000/courses/"

    curl -H "Content-Type: application/json" -X POST -d '{"teacher_id":1, "name":"Second course"}' "127.0.0.1:3000/courses/"
    
    curl 127.0.0.1:3000/courses/1
    ```

  - GET:  /courses/{teacher id}/{course id}

    ```sh
    curl -H "Content-Type: application/json" -X POST -d '{"teacher_id":1, "name":"First course"}' "127.0.0.1:3000/courses/"

    curl -H "Content-Type: application/json" -X POST -d '{"teacher_id":1, "name":"Second course"}' "127.0.0.1:3000/courses/"

    curl 127.0.0.1:3000/courses/1/1
    ```


## 数据库

### 准备工作

1. 依赖

  项目依赖于 `sqlx` 的crate

2. postgres安装

  1. 环境

    ubuntu20.04 + docker

  2. docker 安装postgres镜像

   - 拉取镜像

      ```sh
      docker pull postgres 
      ```
   - 建立容器

      ```sh
      docker run -d --name postgres-test -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres
      # postgres-test 为自定义名称
      # POSTGRES_PASSWORD=postgres 等号后面是设置的密码
      # 5432:5432 是将postgres的默认端口5432映射到本地的5432端口 
      ```

   - 启动容器
      ```sh
      docker start postgres-test
      ```

   - 查看当前运行的容器
      ```sh
      docker ps
      ```

   - 停止容器
      ```sh
      docker stop postgres-test
      ```

   - 删除容器
      ```sh
      docker rm postgres-test
      ```

   - 进入容器的bash shell命令行
      ```sh
      docker exec -it postgres-test bash
      ```

   - 进入容器的postgres shell命令行
      ```sh
      docker exec -it -u postgres postgres-test psql
      ```

### 数据库建表

> 以下在容器内postgres用户的数据库shell执行

1. 创建数据库用户`root`，并设置密码
  ```sql
  create user root with password 'root';
  ```

2. 为用户`root`创建数据库`tutorail`并赋权给`root`
  ```sql
  create database tutorail owner root;
  grant all privileges on database tutorial to root;
  ```


3. (重构前)使用`DataGrip`以`root`用户连接`tutorail`数据库， 开始建表

  ```sql
  create table course
  (
      id         serial,
      teacher_id integer not null,
      name       varchar(140),
      time       timestamp default now()
  );
  ```


4. 重构之后的建表

  ```sql
  drop table course;

  create table course
    (
        id          serial primary key,
        teacher_id  integer not null,
        name        varchar(140),
        time        timestamp default now(),
        description varchar(2000),
        format      varchar(30),
        structure   varchar(200),
        duration    varchar(30),
        price       integer,
        language    varchar(30),
        level       varchar(30)
    );
  ```

