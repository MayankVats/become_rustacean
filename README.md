# Become Rustacean <img src="./README.png" alt="Rust logo" width=50 height=50>

---

## Ownership

Ownership defines how rust manages memory. Some languages have garbage collector which regularly checks for unused value while in some languages the programmer have to explicitly de-allocate the memory.

But, in rust there are set of rules which the rust compiler checks and if the rules are violated then the program won't compile.

The rules are:

- Each value in rust has an owner.
- There can only be one owner at a time.
- When owner goes out of scope the values will be dropped.

Rust does not have garbage collection, when the variable goes out of scope rust deallocates the memory resource owned by that variable.

Types that have a known size at compile time are stored on stack. Types that might need a memory to be allocated to them have their values stored on heap while their metadata is stored on stack.

### Data Race

A data race is when reading from data has the possibility of being out of sync due to the existence of a writer to the data at the same time. This happens often in multi-threaded programming.
