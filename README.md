# ECE 459: Programming for Performance

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Performance](https://img.shields.io/badge/focus-Low%20Latency%20%7C%20High%20Throughput-blue.svg)
![Threads](https://img.shields.io/badge/Parallelism-Multithreaded%2C%20SIMD%2C%20Async-green)
![Tools](https://img.shields.io/badge/Tools-Cargo%2C%20Flamegraph%2C%20perf-lightgrey)
![License](https://img.shields.io/badge/license-MIT-lightgrey)

A portfolio of Rust-based systems projects focused on improving performance through concurrency, parallelism, memory efficiency, and profiling.

---

## Topics Covered

* Multithreading and data-parallel workloads
* Concurrency with locks and channels
* Low-level memory performance and cache optimization
* Profiling and benchmarking (`perf`, flamegraphs)
* Safe systems programming in Rust

---

## Project List

| Folder                                  | Project Title                 | Key Concepts                                              |
| --------------------------------------- | ----------------------------- | --------------------------------------------------------- |
| `00-practice-clones`                    | Ownership + Cloning Practice  | Rust ownership, memory semantics, cloning correctness     |
| `00-practice-lifetimes`                 | Lifetime Annotations          | Borrow checker, lifetimes, static guarantees              |
| `01-rust-basics-and-benchmarking`       | Rust Benchmarks               | Micro-benchmarking, Cargo test, timing functions          |
| `02-threading-and-parallelism`          | Data Parallelism with Threads | `std::thread`, `Arc`, `Mutex`, shared state concurrency   |
| `03-concurrency-and-synchronization`    | Concurrency Control           | Channel messaging, mutexes, thread coordination           |
| `04-performance-analysis-and-profiling` | Profiling & Optimization      | Flamegraphs, `perf`, bottleneck analysis, CPU-bound tasks |

---

## Build and Run

Each project uses Cargo:

```bash
cargo build
cargo run
```

Some may require test flags or profiling tools. Check each folder’s README for details.

---

## Architecture Overview

```
+-------------------------------+
|       Rust Application Layer  |
|  (Threads, Channels, CLI)     |
+-------------------------------+
              |
              v
+-------------------------------+
|    Concurrency Primitives     |
|   Mutex, Arc, Channels        |
+-------------------------------+
              |
              v
+-------------------------------+
|   Runtime and Memory Model    |
|  Ownership, Lifetimes, Safety |
+-------------------------------+
              |
              v
+-------------------------------+
|    Target Hardware Platform   |
|   (CPU, Caches, SIMD, etc.)   |
+-------------------------------+
```

---

## About

These projects highlight practical performance tuning and safe systems-level concurrency in Rust, with an emphasis on profiling, correctness, and real-world resource handling. Ideal for showcasing knowledge of modern low-level systems techniques in a safe language.
