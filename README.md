# c-rust-performance-test
Simple C, C++ and Rust implementations for the sieve of Eratosthenes (search for prime numbers in range 2~100,000,000). The test result shows the Rust program is as fast as the C++ program.

## Compile
```
clang c_prog.c -o c_prog_clang -O3 -lm
gcc c_prog.c -o c_prog_gcc -O3 -lm

clang++ cpp_vect.cpp -o cpp_vect_clang -O3 -lm
g++ cpp_vect.cpp -o cpp_vect_gcc -O3 -lm

rustc -C opt-level=3 rust_prog.rs
rustc -C opt-level=3 rust_unsafe_slice.rs
rustc -C opt-level=3 rust_unsafe_ptr.rs
```

## Test result
To get more significant execution time, an old PC is choosed to run the test. The CPU is Intel(R) Atom (Pineview) CPU D525 @ 1.80GHz (hyper-threading enabled).
```
x86_64-pc-linux-gnu
Ubuntu clang version 14.0.0-1ubuntu1.1
gcc (Ubuntu 11.4.0-1ubuntu1~22.04) 11.4.0
rustc 1.79.0 (129f3b996 2024-06-10)

(clang) c_prog:    5.928332 s
(gcc)   c_prog:    6.111202 s
(clang) cpp_vect:  6.175570 s
(gcc)   cpp_vect:  6.418330 s

rust_prog:         6.184 s
rust_unsafe_slice: 6.335 s
rust_unsafe_ptr:   7.055 s
```

It seems strange that the two Rust programs with unsafe slice/pointer access operations are slower than safe iterator operations. The result indicates that iterators in Rust are well-optimized.
