# Calculate median number

Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

I made this application during study the book - [The Rust Programming Language](https://doc.rust-lang.org/book)

## Build and run
```shell
cargo run
```

Example with odd count of numbers: 

```text
% cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust-median`
Insert an odd number of numbers (min 3 numbers): 
7 2 1 5 6 8 3 2 9
Sorted vector: [1, 2, 2, 3, 5, 6, 7, 8, 9]
Median: 5.0
Mode: 2
```

Example with even count of numbers:
```text
% cargo run 
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/rust-median`
Insert an odd number of numbers (min 3 numbers): 
12 7 3 15 2 1 15 13
Sorted vector: [1, 2, 3, 7, 12, 13, 15, 15]
Median: 9.5
Mode: 15
```