These should be _identical_ versions of Nim and Rust mmap-access-heavy code to test perfomance differences.


Compile the nim version with:
```
    nim c -d:release mmap_test.nim
```

Compile the rust version with:
```
    cargo build --release
```

Both executables can be run with:
```
    ./mmap_test <name_of_large_file> <number_of_accesses>
```
