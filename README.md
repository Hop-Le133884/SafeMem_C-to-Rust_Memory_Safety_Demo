# SafeMem: C-to-Rust Memory Safety Demo

**SafeMem** is an educational project that demonstrates how Rust's ownership model and type system eliminate common memory safety vulnerabilities found in C code. This project features parallel implementations of a simple buffer manager: a deliberately vulnerable C version susceptible to classic memory issues, and a safe Rust version leveraging modern language features. It's ideal for learning about memory safety principles or showcasing Rust's advantages in systems programming.

## Features

- **Vulnerable C Implementation**: A dynamic buffer manager with intentional memory safety flaws
- **Safe Rust Implementation**: A robust version using Rust's `Vec` and ownership model via the `BufferR` struct
- **Comprehensive Test Suite**: Demonstrates memory vulnerabilities in C and Rust's protection mechanisms
- **Real-world Relevance**: Mirrors memory management challenges found in operating systems development

## Why SafeMem?

Memory safety bugs account for approximately 70% of serious security vulnerabilities in C/C++ codebases (according to Microsoft). SafeMem provides a tangible demonstration of how Rust prevents these issues at compile time, serving as a microcosm of why operating system developers are increasingly adopting Rust (e.g., in the Linux kernel and Windows).

## Getting Started

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/)
- **C Compiler**: GCC or equivalent for building the C components
- **Address Sanitizer (ASan)** (recommended): For detecting memory safety issues in C code
  - Debian/Ubuntu: `sudo apt install libasan-dev`
  - Already included with modern GCC/Clang installations

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Hop-Le133884/SafeMem_C-to-Rust_Memory_Safety_Demo.git
   cd SafeMem_C-to-Rust_Memory_Safety_Demo
   ```

2. Build and run:
   ```bash
   cargo run
   ```

## Usage

### Demonstrating Memory Vulnerabilities

The project includes test suites that illustrate the difference between C and Rust approaches:

- **C Vulnerabilities**: Tests in the `tests/` directory trigger buffer overflows, over-reads, and double-free errors in the C implementation
- **Rust Safety**: The Rust implementation (`src/lib.rs`) demonstrates how Rust's ownership system prevents these issues

### Sample Output

```
running 7 tests
test tests::c_double_free ... ok
test tests::c_overflow_test ... FAILED
test tests::c_over_read ... ok
test src::lib::tests::rust_test_buffer_overflow ... ok
test src::lib::tests::rust_test_double_free ... ok
test src::lib::tests::rust_test_over_read ... FAILED
test tests::bindings ... ok

failures:

---- tests::c_overflow_test stdout ----
Testing C vulnerabilities:
Appended 'Hello'
[Crash or garbage output, potentially ASan report]

---- src::lib::tests::rust_test_over_read stdout ----
=== Over-Read Test (Unsafe Rust Implementation) ===
Created buffer with capacity 10
Added 5 bytes
Buffer content (5 bytes):
48 65 6c 6c 6f
Attempting to read at index 5 or more (out of bounds)

test result: PASSED. 5 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.xxs
```

## Project Structure

```
safemem/
├── buffer.rs         # (Potentially a separate Rust buffer implementation)
├── build.rs          # C build script
├── Cargo.lock
├── Cargo.toml        # Rust dependencies
├── config.toml
├── c_src/            # Vulnerable C code
│   ├── buffer.c
│   ├── buffer.h
│   └── test_vulnerabilities.c # C test logic
├── README.md
├── src/              # Rust code
│   └── lib.rs        # Safe Buffer implementation (BufferR struct and tests)
└── tests/            # Rust integration tests, including C vulnerability tests via FFI
    ├── bindings.rs   # FFI bindings to the C code
    ├── c_double_free.rs
    ├── c_overflow_test.rs
    └── c_over_read.rs
```

## How It Works

### C Implementation

- **`buffer_append`**: Lacks bounds checking, enabling buffer overflows
- **`buffer_free`**: Frees memory but allows potential reuse or double-free errors

### Rust Implementation

- **`Buffer::append`**: Uses `Vec` with automatic bounds checking and resizing
- **`Drop` trait**: Ensures safe, single deallocation without manual `free()` calls

### Foreign Function Interface (FFI)

- Rust calls C code via `extern "C"` declarations, enabling side-by-side comparison

## Try It Out

### Testing Memory Safety Issues

1. **Buffer Overflows**: Run tests to see how C allows dangerous memory corruption while Rust safely handles resizing
   ```bash
   cargo test --test c_buffer_overflow
   ```

2. **Over-Read Vulnerabilities**: Compare how C silently reads invalid memory while Rust panics safely
   ```bash
   cargo test --test c_use_after_freed
   ```

3. **Double-Free Errors**: Observe how C allows dangerous double-free while Rust's ownership prevents this
   ```bash
   cargo test --test c_double_free
   ```

### Debugging with Address Sanitizer

For more detailed memory error detection in C code:
AddressSanitizer will detect any memory issues
```bash
cargo test --test c_buffer_overflow -- --show-output
```

## Contributing

Contributions are welcome! Feel free to fork the repository, make improvements, and submit pull requests. Ideas for additional demonstrations, performance optimizations, or better testing are appreciated.

## Acknowledgments

- Inspired by real-world memory safety challenges in operating systems development
- Built to demonstrate the benefits of Rust's ownership model and safety guarantees

---

Created by [HopLe133884] to contrast C's memory management challenges with Rust's safe alternatives.