# Learning Rust Programming language

## Todo

# ToolChain

## OSX Mac Pro

### option 1: brew

```zsh
brew install rust
```

### option 2: rustup

```zsh
curl -sFs https://sh.rustup.rs | sh
```

### verify

```zsh
rustc --verison
```

```
learn_rust git:(main) ✗ rustc --version
rustc 1.69.0 (84c898d65 2023-04-16)
learn_rust git:(main) ✗
```

### Tools of rust

- **Cargo** is the rust compilation manager, package manager and release manager, the most powerful general purpose tools for the rust development
- **rustc** is the rust complier. Usuallly we don't direct invoke this tools, cargo will help us to deal with the complier.
- **rustdoc** is the rust documentation tools, with the nice format comments in the rust source code, rustdoc can help us to produce the HTML format development documents.

## Hello Rust

```zsh
cargo new hello
```

Use cargo to create a new rust project. It will product the following files:

- hello/
- /Cargo.lock
- /Cargo.toml
- /src
- /main.rs
- /target

Cargo.toml is the cargo package management config file, just like nodejs pakcage.json, the project meta data and package dependencies will be configured with this file.
Cargo.lock is the lock file, which will enable cargo to build with reproduciable build production.
src/ directory is the rust source code resides, put all the source code under this dir.
target/ is the directory of the cargo build, the lib and binary will in this directory.

# Ownership of the Rust

Rust memory management has its unique method, the ownership of Rust goven the rust programme memory management.

There are there type of the memory management methods:

- Garbage Collection, like Python and Java, the development don't worry about the memory management.
- Explicitly allocation and release memory, like C/C++, lead to memory leak problme and security exploite
- Ownership Rules in Rust

## Rules of Ownership

- Each Value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of the scope, the value will be dropped.

### Variable and scope

Declar a variable, rust will allocate the memory; when the the variable is out of scope, rust will automatically free the memory of this variable.

## Syntax and Semantics

### Variables Bindings

Variable declaration and assignments:

```rust
fn main() {
    let x = 52; // type is i32, implicit
}
```

Type Annotation

```rust
fn main() {
    let x: i32 = 52; // type is i32, annotated
}
```

Mutability

````rust
fn main() {
    let x: i32 = 52; // type is i32, annotated
    x = 53;
}

the rustc compiler will return
```sh
error: re-assignment of immutable variable `x`
     x = 10;
     ^~~~~~~
````
