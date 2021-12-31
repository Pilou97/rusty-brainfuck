# Rusty Brainfuck

Brainfuck interpreter in rust \
You have to modify the main program to run your own brainfuck program

The default example is fibonnaci

```
cargo run
cargo build
```

Some error are handles by the interpreters
 - overflow in memory (the memory is just an array of unsigned byte)
 - out of memory, if the user use to go outside the memory (memory are limited to 30k)
 - wrong parenthesis (a stack is used to determine if the parenthesis are good or not at parsing time)

