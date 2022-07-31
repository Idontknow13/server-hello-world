# Examples in Rust

Backend Server Examples written in various Rust Web Frameworks.

---

## Functionality

Each example can:

- [x] Send a "Hello World" at endpoint `/` (root)
- [x] Send a customized hello message at endpoint `/` with query string: "name".

## Running

To run, make sure you have the Rust toolchain installed and type:

```$bash
$ cd {framework}-example
$ cargo run
```

on your terminal.

> For rocket-v0.4, type `cargo override set nightly` in order to use the nightly toolchain.

## Frameworks

- [actix-web](https://actix.rs)
- [rocket-v0.4](https://crates.io/crates/rocket)
- [rocket-v0.5rc2](https://rocket.rs)

