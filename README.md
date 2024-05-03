# Felys-Interpreter

**Update**: I've finished the lexer part, you can use `cargo run` to quickly see the AST of a sample program.

I'm currently re-constructing the interpreter, and the eval part will not be available for a while. However, when everything is ready, the new version will be able to handle basic loops and if statement.

### Upstream: [Felys-Project](https://github.com/FelysNeko/Felys-Project)

## Feature

### Typing

- String: `'elysia'`
- Integer: `42` (only support up to Rust `isize`)
- Bool: `true` | `false`

### Variable

Assign value to a variable: `x = 'elysia is my waifu'`

### Evaluation

- Arithmatic: `a+b`, `a-b`, `a*b`, `a/b`, `a%b`
- Comparison: `a>b`, `a>=b`, `a==b`, `a!=b`, `a<=b`, `a<=b`, `a>b`
- Logical: `a&&b`, `a||b`
- Unary: `+a`, `-a`, `!a`

You can also use `+` to concat String.
