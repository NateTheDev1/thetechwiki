Get rid of unused variable warnings:

```rs
#![allow(unused)]
```

All variables made inside of rust are immutable (you cannot change their value)

```rs
    let mut x = 1;
    x = 2; // OK

    let y = 1;
    y - 2; // BAD - mut not specified
```
