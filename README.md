# **Calculation of Pi Using Multithreading in Rust**  

This repository contains examples of Rust programs that calculate the value of π using the Monte Carlo method with multithreading.  

The **Monte Carlo method** works by randomly generating points within a unit square and determining the ratio of points that fall inside a quarter-circle to the total number of points.  

## **Implementations**
There are four implementations in this repository:
- **Native Rust multithreading** using `std::thread`
- **Rayon framework** for parallel execution  

## **System Used for Testing**
These programs were tested on a **Ryzen 9 5950X** CPU.  
By default, they are set to **24 threads** and **1,000,000,000 points**.  

## **How to Modify Thread Count and Points**
To ensure the code runs correctly on your system, manually set the number of threads in each implementation.  

In each `src/main.rs` file, update:

```rust
let num_threads = <your CPU thread count>;
```

You can also change the total number of points:

```rust
let total_points: u64 = <your desired number of points>;
```

## **Benchmarking**
A **benchmarking Bash script** is included in this repository. It:  
- Now portable across Linux, FreeBSD and MacOS
- Runs all four implementations **10 times**  
- Averages their execution time  
- Pre-builds each program using `cargo` and measures only the **binary runtime**
- After completion log file with results is created

[View Benchmark Results](https://raw.githubusercontent.com/Peter-L-SVK/Pi_multithread/refs/heads/main/benchmark_results.log)

## **Dependencies**
Make sure you have the following installed:  
- **Rust and Cargo** (Rust’s package manager)  
- A **Unix-like environment** (Linux, FreeBSD or MacOS) for the bash script, Rust programs work on Windows 

Tested on **Fedora 40**  
