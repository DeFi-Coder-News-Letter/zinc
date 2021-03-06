# Arrays

Arrays are collections of values of the same type sequentially stored in the memory.

Fixed-sized arrays follow the Rust rules. The only exception is the restriction
to constant indexes, that is, you cannot index an array with anything but a
constant expression for now.

Arrays support the index and slice operators, which is explained in detail [here](../../05-operators/06-access.md).

```rust,no_run,noplaypen
let mut fibbonaci = [0, 1, 1, 2, 3, 5, 8, 13];
let element = fibbonaci[3];
fibbonaci[2] = 1;
```
