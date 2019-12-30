# News

[![Build Status](https://travis-ci.com/sysu-2019-mosad-9/News-Backend.svg?branch=master)](https://travis-ci.com/sysu-2019-mosad-9/News-Backend)

SYSU MOSAD 2019 Final Homework

Backend for app **News Reader**, written in Rust

Group Members:

+ Liang Junhua([@Alva112358](https://github.com/Alva112358))
+ Liu Jiahui([@tplish](https://github.com/tplish))
+ Liang Saibo([@dasinlsb](https://github.com/dasinlsb))

## Build & Run

### With Docker

```shell
docker-compose up -d
```

### Without Docker

[Postgres](https://www.postgresql.org) is required

Modify following configuration in `.env`
```
DATABASE_URL=postgres://postgres:postgres@db/newsback
```

To something like this:

```
DATABASE_URL=postgres://postgres:postgres@localhost/newsback
```

Run server in development(or production) mode

```
cargo run
```