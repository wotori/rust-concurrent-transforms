# Rust Multi-threaded Data Processing

This repository contains a Rust application designed to demonstrate efficient multi-threaded data processing. It leverages Rust's powerful concurrency features, including the use of `Arc`, `Mutex`, and dynamic thread management based on the system's available cores. The application processes a set of numerical data through a transformation function, dynamically adjusting the computation to the number of logical cores available.

## Features

- Safe multi-threaded data processing using Rust's ownership and concurrency model.
- Dynamic adjustment of thread count based on available system cores.
- Implementation of the [Collatz conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture) for number transformation.

## Requirements

- Rust 1.77.2 was used during development
