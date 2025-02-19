# Calculation of Pi numerals by multithreding method using Rust

Here are an examples of a Rust program that calculates the value of π using a Monte Carlo method with multithreading. The Monte Carlo method involves randomly generating points in a unit square and determining the ratio of points that fall within a quarter-circle to the total number of points. 

Exaples are running using native multithread and Rayon freamework for multithreading.

My CPU is R9 5950X. \
Programms are set to !!!24 threads!!! and 1_000_000_000 points!!!
To ensure code working properly, set number of threads manually in main.rs inside each of src directory: < let num_threads = >
according to number of threads on your CPU model

You can also change number of points manually in same fasion with: < let total_points: u64 = > 

For benchmark, you can use becnchmarking Bash script in this repo, which rans all of four examples ten times and averaging their run time.
