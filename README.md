# Undefined Behavior in Rust Vector Modification

This repository demonstrates a common source of undefined behavior in Rust: directly manipulating a vector's memory through a raw pointer without careful consideration of the vector's internal state.  The `bug.rs` file shows the problematic code, while `bugSolution.rs` offers a safer and more idiomatic alternative. 

**Understanding the Issue:**
The original code retrieves a raw pointer to the vector's data using `as_mut_ptr()`.  Subsequently, modifying the vector's contents through this pointer without maintaining the vector's internal consistency (length and capacity) can lead to unpredictable behavior, crashes, or data corruption. 

**Solution:**
The improved solution demonstrates how to avoid such issues using Rust's safer mechanisms for vector manipulation.
