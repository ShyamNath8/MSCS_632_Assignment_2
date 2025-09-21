Memory Management Comparison: Rust, Java, and C++

This project demonstrates and compares memory management across **Rust**, **Java**, and **C++** using both conceptual examples and simple measurement programs.  
The goal is to illustrate how each language allocates, uses, and releases memory.


Overview

Three programs were written in Rust, Java, and C++ to explore:

1. **Concept examples** – how each language allocates and frees memory:
   - Rust: Ownership and scope-based deallocation
   - Java: Garbage collection
   - C++: Manual allocation and `delete`

2. **Measurement examples** – allocating arrays of integers and printing memory usage:
   - Rust: Using `sysinfo` crate
   - Java: Using `Runtime.getRuntime()`
   - C++: Using `sizeof` and manual allocation


Requirements

- **Rust** (install via [rustup](https://www.rust-lang.org/tools/install))
- **Java JDK 11+** (download from [Oracle](https://www.oracle.com/java/technologies/javase-downloads.html) or OpenJDK)
- **C++ Compiler** (e.g., `g++` via [MinGW-w64](https://www.mingw-w64.org/) on Windows, or preinstalled on Linux/Mac)

- [Visual Studio Code](https://code.visualstudio.com/) with language extensions for Rust, Java, and C++


How to Run

Rust
```bash
cd rust_memory
cargo run
