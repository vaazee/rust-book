# Notes from the rust book: https://doc.rust-lang.org/book/title-page.html


## Snippets and useful commands

```bash
cargo build

cargo build --release

cargo run 

cargo run --release 

# opens the docs in local browser for crates added as dependencies
cargo doc --open
```

```rust
// You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated. constants cannot be mutable. 

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

```

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.


Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.


Statements vs Expressions
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    //  If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. 
    println!("The value of y is: {y}");
}

```

