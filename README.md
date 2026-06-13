# MathUtils - Rust Math Utilities Library

A simple Rust library providing basic arithmetic operations with proper error handling.

## Features

- Addition (`add`)
- Subtraction (`sub`)
- Multiplication (`mul`)
- Division (`div`) with zero division error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mathutils = "0.1.0"
```

## API Documentation

### Functions

#### `add(a: i32, b: i32) -> i32`

Adds two integers and returns the result.

```rust
use mathutils::add;

let result = add(2, 3);
assert_eq!(result, 5);
```

#### `sub(a: i32, b: i32) -> i32`

Subtracts two integers and returns the result.

```rust
use mathutils::sub;

let result = sub(5, 3);
assert_eq!(result, 2);
```

#### `mul(a: i32, b: i32) -> i32`

Multiplies two integers and returns the result.

```rust
use mathutils::mul;

let result = mul(2, 3);
assert_eq!(result, 6);
```

#### `div(a: i32, b: i32) -> Result<i32, &'static str>`

Divides two integers. Returns `Ok(result)` on success or `Err("Division by zero")` if dividing by zero.

```rust
use mathutils::div;

let result = div(6, 3);
assert_eq!(result, Ok(2));

let result = div(6, 0);
assert_eq!(result, Err("Division by zero"));
```

## Running the Demo

The library includes a demo binary that shows how to use all functions:

```bash
cargo run
```

## Running Tests

To run the unit tests:

```bash
cargo test
```

## License

MIT
