[![Cargo Build & Test](https://github.com/Gamma749/RustCalculator/actions/workflows/ci.yml/badge.svg)](https://github.com/Gamma749/RustCalculator/actions/workflows/ci.yml)

# Rust Calculator
## Hayden McAlister 2022

A basic Reverse Polish Notation calculator written in Rust. Mainly an experiment in developing in Rust, looking at advanced language features, and including continuous integration using GitHub Actions.

---

Basic structure is a struct holding a stack of f64 and a set of operations mapping a stack onto a new stack.

Operations are implemented using a hashmap of closures to associate a static string with an operation (e.g. `"+" => |a,b|{a+b}`). Initially, traits were used to implement generic operations mapping f64 to f64 but this proved unwieldy. One sticking point is the creation of operations at compile time. Currently, a vector of operations is defined at the creation of a new Calculator struct, although I did attempt to create the hashmap at compile time using PHF. This may be worth looking into again.

---

TODO:

- Add CLI launched from `main.rs`