# ftswarm-rs | A Rust implementation of the ftSwarm Protocol

> This project is specifically designed for use with the [ftSwarm](https://elektrofuzzis.github.io/en/index.html), if
> you don't own some of them yet, you can order them [here](https://gundermann-software.de/shop).

## What is the ftSwarm?

The goal of the ftSwarm project is to build small networked controllers for DIY and toy applications. Since they are
networked with each other, they can act like a swarm.

For example, they can act as several independent robots to solve a task together. In larger models, the controllers can
be installed at different positions close to actuators and sensors - together they control the complete model.

Originally, the project was designed to use with _fischertechnik_. Since the mounting grooves are compatible with
Makerbeam profiles, ftSwarms could be used in many DIY projects.

## What is ftswarm-rs?

For controlling the ftSwarm, you have two options: Embed your code into the ftSwarm firmware or use the ftSwarm as a
dumb device and control it from a central computer. This repository is an implementation of the second option.

There's already a Python implementation of the ftSwarm protocol, but I wanted to have a Rust implementation, so
here it is.

## What is the current state of ftswarm-rs?

The project is in an early stage of development. The following features are already implemented:
- [x] Connect to the ftSwarm
- [x] Send commands to the ftSwarm
- [x] Receive data from the ftSwarm
- [ ] Recover on errors

## Using ftswarm-rs

If you're here, you're probably already familiar with Rust. If not, you can find a good introduction to Rust
[here](https://doc.rust-lang.org/book/). To use ftswarm-rs, you can add it as a dependency to your `Cargo.toml`:

```toml
[dependencies]
ftswarm = { git = "https://github.com/Bloeckchengrafik/ftswarm-rs" }
```

To use it in your code, look at the examples in the [crates/ftswarm/examples](crates/ftswarm/examples) directory.


## How can I contribute?

Contributions are welcome! If you are interested in the project, you can help by testing the software, writing
documentation, or by contributing code. If you are interested in contributing, feel free to open an issue or a pull
request. If you plan to add a new feature, it is a good idea to open an issue first to check back with me first,
to avoid that you are working on something that is already in development.

## How is ftswarm-rs licensed?

ftswarm-rs is licensed under the MIT license. You can find the license text in the file `LICENSE` in the root of the
repository.

- - -
Contact: [christian.bergschneider@gmx.de](mailto://christian.bergschneider@gmx.de)