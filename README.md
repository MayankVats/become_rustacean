# Become Rustacean <img src="./README.png" alt="Rust logo" width=50 height=50>

---

## hello_world

- Running the following command: `cargo new hello_world` will create a directory called **hello_world** which is the rust **package**.

- It will create the following directory structure.
  `hello_world/src/main.rs`
  `hello_world/Cargo.toml`

### Cargo

- It it the Rust's build system and package manager.
- It handles tasks such as building our code, downloading the libraries our code depends on and building those libraries.
- It helps us organize our project.

### Cargo.toml

It is the manifest file for the package which contains metadata like name, version and dependencies for the packages. It is like the `package.json` for the `npm` package.

Each section in the file is defined by square brackets `[]`. For example package section is `[package]` which contains information like _name_, _version_ and _edition_. The `[dependencies]` section lists the dependecies, with version number, which current package depends upon.

### Some useful cargo commands

- `cargo build` - this command will create an executable for the `main.rs` in the `target/debug` directory.
  - To run the executable: `./target/debug/hello_world`
- `cargo run` - this command is useful when you want to build and run in a single step.
- `cargo check` - this commands checks that whether our code compiles, it does not create executable.
