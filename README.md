# Overview
`hashmap-count` is a command-line tool that counts the number of occurrences of each integer in a comma-separated list of integers. It then returns a list of the top n integers with the highest counts, where n is a user-specified parameter.

# Installation
To install `hashmap-count`, you need to have Rust and Cargo installed on your system. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once Rust and Cargo are installed, you can install `hashmap-count` by running the following command:
```
cargo install hashmap-count
```

This will download the source code, compile it, and install the binary on your system.

# Usage
To use `hashmap-count`, run the following command:
```
hashmap-count <input>
```
where `<input>` is a comma-separated list of integers.

For example, to count the occurrences of the integers 1, 2, 3, 4, and 5 run the following command:
```
hashmap-count 1,2,3,2,1
```
This will output the following:
```
Result: [(1, 2), (2, 2), (3, 1)]
```
This indicates that the integers 1 and 2 each occurred twice in the input list and the integer 3 occurred once.


# Conclusion
`hashmap-count` is a simple and useful tool for counting the occurrences of integers in a list. It is easy to install and use, and can be a valuable addition to any developer's toolkit.