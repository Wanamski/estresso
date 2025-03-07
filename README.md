# estresso
A simple program to stress test the CPU written in Rust.

## Installing
Make sure, you have the [Rust toolchain](https://www.rust-lang.org/learn/get-started) installed.

Clone this repository to your machine and open a command line in the directory of the project.

In the command line, run 
```
cargo build
```

## Running the program

Run the program using the command 
```
cargo run -- --count <COUNT>
```
where \<COUNT\> represents the number of workers to start.

For example, if you want to stress test a CPU with 8 Threads, run
```
cargo run -- --count 8
```