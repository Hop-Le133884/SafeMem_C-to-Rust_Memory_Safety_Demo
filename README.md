# SafeMem: C-to-Rust Memory Safety Demo

**SafeMem** is a small, educational project that demonstrates how Rust eliminates common memory safety vulnerabilities found in C. It features a simple buffer manager implemented in both languages: a vulnerable C version prone to buffer overflows, over-reads, and double-free errors, and a safe Rust version leveraging ownership and bounds checking. Perfect for learning about memory safety or showcasing Rust’s systems programming strengths!

## Features
- **Vulnerable C Implementation**: A dynamic buffer manager with deliberate memory safety flaws.
- **Safe Rust Implementation**: A reimagined version using Rust’s `Vec` and ownership model, encapsulated in the `BufferR` struct.
- **Test Harness**: Triggers C vulnerabilities and contrasts them with Rust’s resilience through integration tests.
- **OS Relevance**: Mirrors memory management challenges in operating systems.

## Why SafeMem?
Memory safety bugs (e.g., buffer overflows) account for ~70% of serious security vulnerabilities in C/C++ codebases (per Microsoft). SafeMem shows how Rust prevents these at compile time, making it a toy example of why OS developers are eyeing Rust (e.g., Linux kernel, Windows).

## Getting Started

### Prerequisites
- **Rust**: Install via [rustup](https://rustup.rs/) (`cargo` required).
- **C Compiler**: `gcc` or equivalent for the C component.
- **Address Sanitizer (ASan)** (optional but recommended): For detecting memory safety issues in C (`sudo apt install libasan-dev` on Debian/Ubuntu, typically included with modern GCC/Clang).

### Installation
1. Clone the repo:
   ```bash
   git clone [https://github.com/Hop-Le133884/SafeMem_C-to-Rust_Memory_Safety_Demo.git](https://github.com/Hop-Le133884/SafeMem_C-to-Rust_Memory_Safety_Demo.git)
   cd SafeMem_C-to-Rust_Memory_Safety_Demo

2. Build and run:
   ```bash
   cargo run
   ```

### Usage
- **C Vulnerabilities**: The test suite (tests/) includes tests (e.g., c_overflow_test.rs, c_double_free.rs, c_over_read.rs) that call the vulnerable C code to trigger buffer overflows, over-reads, and double-free errors. Expect test failures and potential crashes or undefined behavior when these tests are executed.
- **Rust Safety**: The Rust implementation (src/lib.rs and tests within tests/bindings.rs and potentially others) demonstrates Rust's memory safety through its ownership system and the use of Vec which handles memory management automatically and performs bounds checking. Tests like rust_test_buffer_overflow, rust_test_double_free, and rust_test_over_read in src/lib.rs showcase this.
- **Output Example**:
running 4 tests
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
thread 'src::lib::tests::rust_test_over_read' panicked at 'index out of bounds: the length is 5 but the index is 5', src/lib.rs:110:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::c_overflow_test
    src::lib::tests::rust_test_over_read

test result: FAILED. 5 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.xxs


## Project Structure
 safemem/
├── buffer.rs         # (Potentially a separate Rust buffer implementation - clarify if needed)
├── build.rs          # C build script
├── Cargo.lock
├── Cargo.toml        # Rust dependencies
├── config.toml
├── c_src/            # Vulnerable C code
│   ├── buffer.c
│   ├── buffer.h
│   └── test_vulnerabilities.c # C test logic (may be demo seperately if in case)
├── README.md
├── src/              # Rust code
│   └── lib.rs        # Safe Buffer implementation (BufferR struct and tests)
└── tests/            # Rust integration tests, including C vulnerability tests via FFI
   ├── bindings.rs   # FFI bindings to the C code
   ├── c_double_free.rs
   ├── c_overflow_test.rs
   └── c_over_read.rs

## How It Works
- **C Version**:  
  - `buffer_append`: No bounds checking—overflows possible.  
  - `buffer_free`: Frees memory but allows reuse or double-free.  
- **Rust Version**:  
  - `Buffer::append`: Uses `Vec` with bounds checks and overflow protection.  
  - `Drop`: Ensures safe, single deallocation—no manual `free` needed.  
- **FFI**: Rust calls C via `extern "C"` to compare side-by-side.

## Try It Out
- ## C Version:
c_src/buffer.c: Implements a buffer with potential for memory safety issues.
c_src/buffer.h: Header file for the C buffer.
tests/: Rust tests that call C functions (via bindings.rs) in c_src to trigger vulnerabilities. test_vulnerabilities.c might contain similar logic or be used for direct C testing.
Vulnerabilities like buffer overflows (buffer_append without bounds checking), over-reads, and double-free are intentionally present.
- ## Rust Version:
src/lib.rs: Implements a safe buffer (BufferR struct) using Vec for automatic memory management and bounds checking. Includes Rust-specific tests (rust_test_buffer_overflow, rust_test_double_free, rust_test_over_read) to demonstrate safe behavior. The Drop trait ensures safe, single deallocation for the LowLevelBuffer.
- ## FFI:
tests/bindings.rs: Defines the foreign function interface (extern "C") that allows Rust to call functions in the compiled C library.
Rust tests in tests/ use these bindings to interact with the vulnerable C code and demonstrate the safety differences.
Try It Out
The tests/ directory contains specific Rust integration tests designed to expose the C vulnerabilities. You can run all tests using cargo test.

Buffer Overflows: The tests/c_overflow_test.rs file likely calls the C buffer_append function with data exceeding the buffer's capacity. Observe the test output and potential crashes or reports from Address Sanitizer. The rust_test_buffer_overflow in src/lib.rs shows how Rust's Vec handles this safely by automatically resizing.

Over-Read: The tests/c_over_read.rs file likely attempts to read beyond the bounds of the C buffer. The rust_test_over_read in src/lib.rs demonstrates how Rust's indexing on Vec will panic (safely terminate with an error message) instead of allowing a potentially dangerous out-of-bounds read.

Double-Free: The tests/c_double_free.rs file likely calls the C buffer_free function twice on the same buffer. This should lead to a crash or an error reported by Address Sanitizer. The rust_test_double_free in src/lib.rs shows how Rust's ownership and Drop trait prevent this by ensuring memory is deallocated exactly once when the BufferR goes out of scope.

- ## Debugging with Address Sanitizer: To detect memory errors in the C code, run the tests with Address Sanitizer enabled (if your compiler supports it):

Bash

cargo test -- --nocapture -Z sanitizer=address

## Contributing
Feel free to fork, tweak, or suggest improvements! Open an issue or PR if you’ve got ideas—extra vulnerabilities to demo, performance tweaks, or better test cases are welcome.

## Acknowledgments
Inspired by real-world memory safety challenges in OS development.
Built with Rust’s fearless concurrency and ownership model.

Made by [HopLe133884] to explore C’s chaos and Rust’s calm.
