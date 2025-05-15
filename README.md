# ECE 459: Programming for Performance — Rust Systems Portfolio

This repository contains a collection of assignments and exercises from **ECE 459: Programming for Performance**, focusing on writing high-performance software using Rust.

The course explores core performance principles including:
- Low-latency and high-throughput programming
- Multithreading and synchronization
- Memory safety and concurrency using Rust
- Profiling, benchmarking, and performance analysis
- Real-world application of scalable software design

Each folder contains an independent project or practice exercise using Rust, with Makefiles or Cargo-based builds and instructions.

## Project Overview

| Folder                             | Title                                      | Focus Area                                      |
|------------------------------------|--------------------------------------------|-------------------------------------------------|
| 00-practice-clones                | Rust Ownership Practice (Cloning)          | Lifetimes, borrow checker, memory ownership     |
| 00-practice-lifetimes             | Rust Lifetimes Practice                    | Lifetimes, compiler enforcement                 |
| 01-rust-basics-and-benchmarking   | Rust Basics & Micro-Benchmarking           | Rust setup, performance timing, macros          |
| 02-threading-and-parallelism      | Threading & Parallel Workloads             | Rust threads, shared memory, Arc/Mutex          |
| 03-concurrency-and-synchronization| Concurrency with Synchronization           | Race condition avoidance, locking strategies    |
| 04-performance-analysis-and-profiling | Performance Profiling & Optimization    | Flamegraphs, benchmarking, optimization         |

---

### 🛠 How to Build & Run

Each assignment uses Rust and is built using Cargo:
```bash
cargo build
cargo run
```

Profiling examples may require additional tools like `perf`, `flamegraph`, or `valgrind`.

---

### 💡 Why This Matters

These projects demonstrate a strong command of:
- **Rust systems programming**
- **Low-level performance debugging**
- **Safe concurrency design**

This makes the repo ideal to showcase in a resume or interview portfolio targeting backend, systems, or performance engineering roles.

