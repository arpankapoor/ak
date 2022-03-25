# ak

a (very) basic array programming lang

# build

- install Rust nightly, `rlwrap` and run `make`

# what works

```q
 12*23-4   // no operator precedence - right-to-left evaluation
228
 a:1 2 3   // assign the list 1 2 3 to a
1 2 3
 !10       // ! is the til function - produces a list from 0 to n-1
0 1 2 3 4 5 6 7 8 9
 a:100+!3  // add 1 to each element of the list 0 1 2
100 101 102
 a*a       // vector multiplication
10000 10201 10404
 a*b:1 2 3 // assignment is an expression too
100 202 306
 `hello`world  // symbols: interned strings
`hello`world
```

# todo

- parse functions
- `5*[1;2]` - brackets should bind tighter
- `-*[2;4]` -8
- buddy memory allocator
- learn bytecode
