# Intro Rust Data Umbrella

## How to install Rust

- Go to https://rustup.rs
- Run it online:
  - https://replit.com
  - https://gitpod.io

## Environment

- Any code editor with Rust Syntax Support
  - micro-editor, vim, nvim, kate, geany
- VSCode is recommended with Rust Analyzer extension
- Rust based editors like Lapce and Helix

## Hello World and RustC

To get started we can create file called `hello.rs`

```rust
fn main(){
	println!("Hello, World!");
}
```

- The Rust source code has the `.rs` extension
- The entry point of the program is the `main` function
- We use `println!` macro to print contents to stdout

Compile with Rustc

```bash
rustc hello.rs
```

The compiler will generate a binary with the same name without the extension and you can run it.

```console
$ ./hello
Hello, World!
```

When changing the source code on `hello.rs` the compilation will need to happen again.

## Cargo and Project

On a bigger project you will need to manage more files and eventually external dependencies and to make it easier Rust comes with a Project and Dependency manager called `cargo`.

Creating a new project.

```console
$ cargo new hello-world
     Created binary (application) `hello-world` package
```

Now we can `cd hello-world` and inside that folder is our Rust Project Workspace.

The starting workspace for a binary application is composed by a project config file called `Cargo.toml` which holds all the metadata for the project and its dependencies.

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

and also there is a folder named `src` with a single `main.rs` inside it, `src/main.rs` is the main module and serves as entry point for the project and it comes prefilled with the Hello World program.

```rust
fn main() {
    println!("Hello, world!");
}
```

As we are now working with a Rust Project we can use `cargo` and the tool to build, run and perform all administration tasks on the project, start by trying `cargo run`

```console
$ cargo run
   Compiling hello-world v0.1.0 (../rust-intro/hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33
     Running `target/debug/hello-world`
Hello, world!
```

This command compiles and then runs the code, if you want to only compile without running you can run `cargo build` and then cargo will create a binary in the `./target/debug/` folder.

```bash
./target/debug/hello-world
Hello, world!
```

Cargo can also format your code following the Rust style guide and turn this:

```rust
fn main() { println!( "Hello, world!" ) ; }
```

Using

```bash
cargo fmt
```

into this

```bash
fn main() {
    println!("Hello, world!");
}
```

And there are extre tooling we can add to cargo such as
a tool to automatically compile and run every time the
source code changes.

```bash
cargo install cargo-watch
```

Open a second terminal on the same folder and run

```bash
cargo watch -x run
```

Caargo watch will detect changes when you save the source files and run automatically.

## Variable definition

Defining variables is a trivial activity in many programming languages, you just use the assignment statement and point a value to a given name, in Rust the statement for doing this is `let` and you can clear all the contents of `src/main.rs` and try:

```rust
let x = 5;
```

You gonna see your first error form the compiler

```rust
error: expected item, found keyword `let`
 --> src/main.rs:1:1
  |
1 | let x = 5;
```

That happens because Rust doesn´t allow us to define variables using `let` in the global scope of the program, `let` can be used only inside a well defined scope like a function and there is a reason for that: Memory management.


### RAII

To avoid memory leaking the program must free all the space it claimed to use in the computer memory.

There are languages where you need to allocate and deallocate values from memory manually, for example calling `delete x;` at some point in the program.

There are other languages that uses Garbage Collector, which is a procedure that runs from time to time and detects allocated values that are no longer being used in the program and then cleans it from the memory.

Rust doesn´t use manual memory management, neither garbage collector, Rust addopts a memory management inspired by the RAII pattern, which will define the lifetime of a value according to the scope where it was acquired.


```rust
fn main() {
  let x = 5;  // lifetime for `x` starts
  // more code here
  // ...
}  // lifetime for `x` ends
```

So Rust will automatically free the memory whenever the variable goes out of scope and its lifetime ends.

### Unused variables

Running the above program will result in a warning being printed to the console

```bash
warning: unused variable: `x`
 --> src/main.rs:2:9
  |
2 |     let x = 5;
  |         ^ help: if this is intentional, prefix it with an underscore: `_x`
```

Again, the compiler will tell us exactly what we need to do if we want to avoid the warnings, for now we will just ignore the warnings and pay attention only to the errors.

### Type inference

In most of the cases Rust will be able to infer the type of the variable by its literal or context, as in the example `let x = 5;` where we have not defined an explicit type, in this case, Rust will define `x` as an `i32` by default, we can explicitly set the type if we need.

```bash
fn main() {
    let x: i8 = 5;  // Define x as an integer of 8 bits
}
```

### Printing numbers

We can print numbers to the stdout by using the `println!` macro with variable formatting.

```rust
fn main() {
    let x: i8 = 5;
    println!("{x}");
    // or
    println!("{}", x);
}
```

### Mutability

Now lets try to mutate the number x and then print it again.

```rust
fn main() {
    let x: i8 = 5;
    println!("{x}");
    x = x * 2;  // HERE we try to change the value of x
    println!("{}", x);
}
```

And then we have a very informative error message

```rust
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x: i8 = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("{x}");
4 |     x = x * 2;
  |     ^^^^^^^^^ cannot assign twice to immutable variable

```

By default, variables in Rust are all immutable so we just need to make what
the compiler is telling us to do, add `mut` before the `x`.

```rust
fn main() {
    let mut x: i8 = 5;
    println!("{x}");
    x = x * 2;
    println!("{}", x);
}
```

And now it prints

```bash
5
10
```

SIDE NOTE: To make the code better we can use the assign operators.

```bash
fn main() {
    let mut x: i8 = 5;
    println!("{x}");
    x *= 2;  // instead of x = x * 2;
    println!("{}", x);
}
```

### Strong Typing

We can mutate a variable, however Rust ensures strong typing, that means we can
only mutate to the same type.

```rust
fn main() {
    let mut x: i8 = 5;
    println!("{x}");
    x = "Data Umbrella";  // mutate to a string value
    println!("{}", x);
}
```

The compiler again gives a meaninful message

```rust
error[E0308]: mismatched types
 --> src/main.rs:4:9
  |
2 |     let mut x: i8 = 5;
  |                -- expected due to this type
3 |     println!("{x}");
4 |     x = "Data Umbrella";
  |         ^^^^^^^^^^^^^^^ expected `i8`, found `&str`
```

So what to do?

### Shadowing

We can shadow the existing variable by defining a new one from the scratch and
reusing the same name, simply by using the `let` keywork again.

```rust
fn main() {
    let mut x: i8 = 5;
    println!("{x}");
    let x = "Data Umbrella";  // `let` here shadows previous variable
    println!("{}", x);
}
```

As this new variable is a completely new value in the memory, rust will define it
again and its type now will be infer as `&str` (string reference)

NOTE: We will see warnings about the unused `mut`

### Enclosing scopes

Shadowing also works for enclosing scopes, inside any function we can create
a new scope by simply starting a new block with `{}` or by creating an inner
function or closure.

```rust
fn main() {
    let x: i8 = 5;   // `x` on the main scope defined here
    println!("{x}"); // value 5 is printed
    {
        let x = x * 10;    // `x` on the new scope defined here
        println!("{}", x); // value 50 printed
    }  // the inner x is dropped from stack here
    println!("{x}");  // now `x` is back to the value 5
}  // `x` is cleaned up here
```

### Constants

There is one kind of variable that can be declared on the global scope of the
program and is used to hold values that do not change during the runtime and
have its value known at compilation time.

- Contstants are declared with `const` keyword
- Type definition must be explicit
- Do not allow mutation
- Do not allow reassignment

```rust
const SECONDS_IN_HOURS: u16 = 60;

fn main() {
    let hours = 24; // from dynamic input
    let total = hours * SECONDS_IN_HOURS;
    println!("there are {total} seconds in {hours} hours");
}
```

## Data Types

Rust has a good set of primitive scalar and compound data types

The scalars are:

- integer (signed and unsigned) `u8, i32` - `8, 140085`
- floating point `f32, f64` - `43.3, 56.7`
- boolean `bool` - `true, false`
- char `char` - `'a', '😃'`

The compounds are

- tuple - `(1, 2.3, true, 'a')`
- array - `[1, 2, 3, 4, 5]`

### Integers

| bits | signed | unsigned |
|------|--------|----------|
| 8    | i8     | u8       |
| 16   | i16    | u16      |
| 32   | i32    | u32      |
| 64   | i64    | u64      |
| 128  | i128   | u128     |
| arch | isize  | usize    | <- vary on platform arch

#### signed
range:  -(2ⁿ⁻¹) até 2ⁿ⁻¹ - 1
i8: -128 até 127  [-(2⁷) até 2⁷ - 1]

#### unsigned
range: 0 até 2ⁿ - 1
u8: 0 até 255 [0 até 2⁸ -1]

#### Inference

let x = 5;  // inferred i32

#### Typing

let x: u8 = 10;
let x: 10_u8;

#### Overflow

let x: u8 = 10;
let y: u8 = x - 20;
            ^^^^^^ attempt to compute `10_u8 - 20_u8`, which would overflow

#### Literals

| Literal        | Example     |
|----------------|-------------|
| Decimal        | 98_222      | underscore
| Hex            | 0xff        |
| Octal          | 0o77        |
| Binary         | 0b1111_0000 |
| Byte (u8 only) | b'A'        |

### tuple

Tuple is the basic compound type in Rust

- Can store different types
- Have fixed size

```rust
let tup = (1, 2, 3, 4);
```

- Type is defined by structure
- Acessed by index via dot

```rust
let tup: (bool, i32, f64, char) = (true, 10, 2.1, 'a');
println!("{}", tup.3);
```

- Can be mutable

```rust
let mut tup = (true, 10, 2.1, 'a');
println!("{}", tup.3);
tup.3 = 'b';  // mutation only if the same type
println!("{}", tup.3);
```

- Allow pattern matching destructuring

```rust
let tup = (true, 10, 2.1, 'a');
let (flag, integer, float, letter) = tup;
println!("{flag} - {integer} - {float} - {letter}");
```

### Array

- Fixed size
- Single type
- type inferred by its structure
- acessed by index via subscription `[]`

```rust
let array: [i32; 4] = [1, 2, 3, 4];
println!("{}", array[0]);
```

- can be mutable

```
let mut array: [i32; 4] = [1, 2, 3, 4];
array[0] = 99;
```

## Stack, heap

There are 3 regions on a program's RAM memory

------------
  Stack
------------
   |
 free memory
   |
------------
   Heap
------------
  Static
------------

### Static

Init: When program starts running
Contents: Program binary, Static Variables, String literals
Size: Fixed
Lifetime: Whole Program
Cleanup: When program terminates

### Stack

Init: A new isolated frame is created for every function call
Contents: Function arguments, local variables (all known size at compile time)
Size: Dynamic with upper limit (can cause stack overflow error)
Lifetime: Each function
Cleanup: When function returns
Note: Each thread has its own stack

### Heap

Init: When main function starts
Contents: Values that live beyond function calls
          Values accessible by other threads
          Large Values
          Values with size unknown at compile time
Size: Dynamic (up to mem size) - (reallocation, fragmentation)
Lifetime: Defined by the programmer / language
Cleanup: Manual (or automatic using RAII or GC)
Note: All threads on the three shares the same heap

## Strings

Now that we know about the memory regions lets take a look on how Rust
manages string types.

There are 2 basic string types in Rust

### String Literals (a.k.a &str pronouce `str`)

When we define a variable

```rust
let name = "Bruno";
```

Rust at compile time needs to know the size of the string, so it can store
it in the static memory along with the program binary.

The type is `str`, however `str` by itself doesn't have a size so we can't use
it directly, we need a reference to the static memory location so the reference
can be sized so Rust infers the type as being.

```rust
let name: &str = "Bruno";
println!("Hello {name}");
```

This kind of variable is also known as `String slice` or `String Reference`

- immutable
- static lifetime

### String (a.k.a Dynamic String)

When we work with Strings that are not known at compile time, e.g: read input
from the user, load data from a database or an API etc, then we need to alocate
space in the heap memory because types stored in the heap can have a dynamic size


```rust
fn main() {
    let name = "Bruno".to_string();
    println!("Hello {name}");
}
```

Rust will infer the type above as `name: String`

As `String` is allocated on the heap it can be mutable and have a dynamic size.

```rust
let mut name = String::new();  // static method call
name.push_str("Hello");
name.push_str(" ");
name.push_str("Bruno");
println!("{name}");
```

One important thing to note is that strings are not a sequence of letters,
strings are a sequence of unicode codepoints, so the size of a string might not
be what it seems to be.

```rust
fn main() {
    let name = "Bruno";
    println!("{}", name.len());  // 5

    let symbols = "🦀😃";
    println!("{}", symbols.len());  // 8
}
```

There are ways to count **chars**

```rust
fn main() {
    let name = "Bruno";  // 5
    println!("{}", name.chars().count());

    let symbols = "🦀😃";  // 2
    println!("{}", symbols.chars().count());
}
```

Other ways to initialize a dynamic string

- `"text".to_string()`
- `"text".to_owned()`
- `let x: String = "text".into()`
- `String::new()`
- `String::from("text")`


## Console Input



## Operations and Type Casting
## Control Flow
## Functions and Borrowing
## Custom Types
## A CLI Program
