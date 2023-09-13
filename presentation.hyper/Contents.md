# Share Code with Rust
## A introduction for Swift Developers

---


:: block_alignment: center
### Clone the repository 


![](F67CED47-0300-4B2B-91F4-4C1E5528E981.png)


`https://github.com/terhechte/nsspain2023workshop `

--- :: master: regular1


:: block_alignment: center
## Feel invited to work in pairs!

--- :: master: regular1

## Disable Github Copilot!

***

# Me:


:: size: 150
- https://twitter.com/terhechte
- https://mastodon.social/terhechte
- https://github.com/terhechte
- http://instagram.com/terhechte

***

# Why Rust for Cross Platform 



***

- I'll be giving a talk about this tomorrow
- I'll give a super short overview, but a much better explanation tomorrow
- Tomorrow, I'll also compare against other solutions such as React Native or C++

***

## Cross Platform

:: type:: all
- No Home: Not written for any specific platform
- Available packages oftentimes fully cross platform too
- Linux, BSD, WASM, Nintendo Switch, Arduino, Microcontrollres, PowerPC, MIPS, Fuchsia, Windows 95

--- :: master: title2

:: size: 120
![](CEBDEF01-DED6-4CC7-9090-8AC5855891A4.jpg)


*** :: master: code

### Developer Experience

:: type:: all
- Good Auto Completion for many editors
- Intellij Rust is also really really good
- Excellent Testing experience
- Fast compile times (well, compared to Swift)
- Exceptional package manager (`cargo`)

--- :: master: title1

# Packages

- Swift Package Index: 6,138
- Rust: 123,148
- Python: ~470,000

***


:: type:: all
- PSD / TTF / Docx Parsers, 
- WASM runtimes
- CRDT's (Conflict-free Replicated Data Type)
- Raft (Reliable, Replicated, Redundant, And Fault-Tolerant Consensus)
- Very fast text search
- Great parser combinators (parsing data)
- All kinds of Algorithms

--- :: master: title1



### One language, many Platforms

:: type:: all
- C / C++
- Ruby
- Python
- Dart
- Swift
- Kotlin / JVM
- Javascript / Webassembly
- Go (cgo via FFI)
- PHP
- Erlang

***

# Language Introduction

***

# Hard to learn a language in 2 hours
## Will skip things

***

- Much like Swift
- Imperative & Functional
- Pattern Matching
- Advanced Enums
- Protocols with Associated Types
- Value Types
- Async
- Macros
- Optionals, Results & Error Handling

***

You can find the next slides in the repository. So you don't need to take pictures. You can use the slides as a reference.


--- :: master: title2

## Cargo

``` sh
cargo new
cargo check
cargo run 
cargo build 
cargo test
cargo build --release 
cargo run --release 

```

--- :: master: code

## `cargo --help`

``` sh
cargo search graphql
cargo doc --open
cargo bench 
cargo update
cargo clean
```

--- :: master: code

## First steps

``` sh
sh$ cargo new my-fancy-project
sh$ ls .
my-fancy-project
sh$ 
```

--- :: master: code



## Structure of a binary Rust project

:: theme: Solarized (dark)
``` sh
Cargo.toml
Cargo.lock
src/
  main.rs
target/
  debug/
  release/
```

--- :: master: code

## Structure of a library Rust project

:: theme: Solarized (dark)
``` sh
Cargo.toml
Cargo.lock
src/
  lib.rs
```

--- :: master: code

## Cargo.toml

``` toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"
serde = { version = "1.0.137", features = ["derive"]}
eyere = "*"
```

--- :: master: code





:: size: 70
| Rust  | Swift |
|-------|-------|
| String | String  |
| usize | UInt  |
| isize | Int   |
| i8    | Int8  |
| i32    | Int32  |
| u8    | UInt8  |
| u32    | UInt32  |
| bool    | Bool  |
| f32    | Float  |
| f64    | Double  |
    

--- :: master: regular1




# Functions

--- :: master: regular1




:: label: Swift
``` swift

func example(content: String) -> String {
    content
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

fn example(content: String) -> String {
    content
}
    
```

*** :: alignment: left



:: label: Swift
``` swift

func example(content: Int) -> Int {
    let output = content * 2
    return output
}

example(content: 42)
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

fn example(content: usize) -> usize {
    let output = content * 2;
    output
}

example(42)
    
```

*** :: alignment: left



:: label: Swift
``` swift

func example(content: String) -> String {
    var output = content * 2
    output += 50
    return output
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

fn example(content: usize) -> usize {
    let mut output = content * 2;
    output += 50;
    output
}
    
```

*** :: alignment: left



:: label: Swift
``` swift

func example<T>(content: T) -> T {
    content
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

fn example<T>(content: T) -> T {
    content
}
    
```

*** :: alignment: left



:: label: Swift
``` swift

func compareTwo<T: Equatable>(a: T, b: T) -> Bool {
    a == b
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

fn compare_two<T: Eq>(a: T, b: T) -> bool {
    a == b
}
    
```

*** :: alignment: left



:: label: Swift
``` swift

func compareTwo<T: Equatable>(a: T, b: T) -> Bool 
where T: Equatable {
    a == b
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

fn compare_two<T>(a: T, b: T) -> bool
where T: Eq {
    a == b
}
    
```

*** :: alignment: left



# Closures

--- :: master: regular1




:: label: Swift
``` swift

callAction({ a, b in
    a * b
})

callAction {
    $0 * $1
}

func takesClosure(action: (Int) -> bool)
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

call_action(|a, b| {
    a * b
})

call_action(|a, b| a * b)
// `impl` in Rust => `some` in Swift 
fn takes_closure(action impl Fn(usize) -> bool)
    
```

*** :: alignment: left



# Printing

--- :: master: regular1




:: label: Swift
``` swift

let o = 42
print(o)
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

let o = 42;
println!("{}", o);
    
```

*** :: alignment: left



:: label: Swift
``` swift

let o = 42
print("\(o)")
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

let o = 42;
println!("{o}");
    
```

*** :: alignment: left



:: label: Swift
``` swift

let o = 42
print("\(o)")
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

let o = make_complex_type();
println!("{:?}", &o);
    
```

*** :: alignment: left



# Arrays / Vectors

--- :: master: regular1




:: label: Swift
``` swift

let array = [1, 2, 3]
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

let array = [1, 2, 3];
    
```

*** :: alignment: left


:: label: Swift
``` swift

// Dynamic array
var array = [1, 2, 3]
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

// Dynamic array
let mut array = vec![1, 2, 3];
array.push(7);
array.push(8);
    
```

*** :: alignment: left



:: label: Swift
``` swift

let map = [
    5: "My favorite book."
]
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

let mut map = HashMap::new();
map.insert(5, "My favorite book.");
    
```

*** :: alignment: left



# Tuples

--- :: master: regular1




:: label: Swift
``` swift

let tuple = ("a", 5, 42.0, true)
print(tuple.0)
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

let tuple = ("a", 5, 42.0, true);
println!(tuple.0);
    
```

*** :: alignment: left



# Control Structures

--- :: master: regular1




:: label: Swift
``` swift

if 5 > 10 {
    return 20
} else {
    return 30
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

if 5 > 10 {
    return 20;
} else {
    return 30;
}
    
```

*** :: alignment: left



:: label: Rust
``` rust

fn test() -> usize {
    // The last expression is returned
    let a = 4 + 2;
    let b = a * 3
    b
}
    
```

***




:: label: Rust & Swift 5.9
``` rust

let hello = if a > b {
    23
} else {
    24
};

```

***




:: label: Swift
``` swift

for x in array {
    print(x)
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

for x in array {
    println!(x);
}
    
```

*** :: alignment: left

# Strings

- Rust: `String` -> Swift: `String`
- Rust: `&str` ~> Swift: `Substring`
- `String` is a allocated instance, `&str` is a reference to a `String` or part of a `String`
- A `String` is composed out of `char`s. You can get them via `string.chars()`

---


## Unicode

- A `String` is composed out of `char`s. You can get them via `string.chars()`
- A Rust `char` is **not** a *Unicode Grapheme Cluster* (like in Swift). Instead, its a *Unicode scalar Value*. 
- The Family emoji:
- Rust: 👨 👩 👧 👦
- Swift: 👨‍👩‍👧‍👦

--- :: master: regular2

# First task

--- :: master: title1

## Tip: Debug Printing

You can use `dbg!(...)` for debug printing

``` rs
// It will just pipe the data through
let my_thing = dbg!(call_thing());
```

--- :: master: regular2

## Tip: Debugger

- If you installed the debugger VSCode extension, you have a proper debugger
-  Just press `F5` to run the app in the debugger

--- :: master: regular2

# Demo

- Docs
- Crates.io
- VSCode

--- :: master: title1

# Project 1

- Read characters from a file
- Figure out how many symbols are math symbols
- Some code has already been written
- See `Readme.md`
- Total: 20min
- After 12min I will share the password for the solution so you can compare your solution if you want


--- :: master: regular2

# Enums

--- :: master: regular1




:: label: Swift
``` swift

enum Gender {
    case male
    case female
    case neutral
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

enum Gender {
    Male,
    Female
    Neutral
}
    
```

*** :: alignment: left



:: label: Swift
``` swift

enum Example {
    case first(String)
    case second(Float)
    case third(Int, String)
}
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

enum Example {
    First(String),
    Second(f32),
    Third(i32, String),
}
    
```

*** :: alignment: left



:: label: Rust
``` rust

enum Example {
    First { value: String },
    Second { value: f32 },
    Third { value: i32, content: String }
}
    
```

***




:: label: Swift
``` swift

enum Gender { case male, female }
let x = Gender.female
    
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

enum Gender { Male, Female }
let x = Gender::Male;
    
```

*** :: alignment: left



# Optionals

--- :: master: regular1




:: label: Swift
``` swift

enum Optional<T> {
    case some(T)
    case none
}
        
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

enum Option<T> {
    Some(T)
    None
}
        
```

*** :: alignment: left



:: label: Rust
``` rust

if let Some(unwrapped) = optional_value {
}

if let Option::Some(unwrapped) = optional_value {
}
        
```

***




# Pattern Matching

--- :: master: regular1



:: label: Rust, animate: (0.50;move;bottom)
``` rust

let o = Some(Value::Number(42))
match read_user() {
    Some(Value::Number(n)) if n > 10 => println!(n),
    Some(o) => println!("{o}"),
    _ => ()
}
        
```

*** :: alignment: left



:: label: Swift
``` swift

guard case let .number(n) = readUser else { 
  return 
}
        
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

let Some(Value::Number(n)) = read_user() else {
  return
};
        
```

*** :: alignment: left

## Implicit Returns

:: label: Rust
``` rust

fn ex() -> Option<()> {
    let lastChar = user?.name?.chars().first?.int;
}
        
```

***




# Structures

--- :: master: regular1




:: label: Rust, animate: (0.50;move;bottom)
``` rust

struct Hello {
    first: String,
    second: i32
}
        
```

*** :: alignment: left



:: label: Rust
``` rust

struct Hello {
    first: String,
}
impl Hello {
    // self.print_intro();
    fn print_intro(&self) {
        // In rust, `self.` is required
        println!("Hello {}", self.first)
    }

    // static fn: Hello.print_intro();
    fn print_intro() {
      ...
    }
}
        
```

***


:: label: Swift
``` swift

struct Hello: Codable {
    let first: String
    var second: Int
}
        
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust
use serde::{Serialise, Deserialise};

// Requires `serde` dependency
#[derive(Serialize, Deserialize)]
struct Hello {
    first: String,
    second: i32
}
        
```

*** :: alignment: left

## `Derive` adds functionality via macros

```
struct Klaus: Coddle, Equatable, Hashable {...}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash]
struct Klaus { ... }
```


***

# `Copy`, `Clone`

- `Copy`: Only for value types (Float, Integer, Bool)
- `Clone`: For heap memory types (String, Vec, HashMap, etc)

``` rs
#[derive(Copy)]
struct Test {
  value: i32
}

#[derive(Copy)]
struct Test2 {
  value: String
  Value2: i32
}
```


---

## `Debug` & `Default`
:: label: Rust
``` rust

#[derive(Debug, Default)]
struct Hello {
    first: String,
    second: i32
}

let x = Hello::default();
println!("{:?}", x);
        
```

***




# Error Handling

--- :: master: regular1




:: label: Rust
``` rust

fn parse_me(content: String) -> Result<i32> {
    let parsed: i32 = content.parse()?;
    parsed *= 100;
    Ok(parsed)
}
    
```

***




:: label: Rust
``` rust

fn parse_me(content: String) -> Option<i32> {
    let parsed: i32 = content.optional_fn().ok()?;
    parsed *= 100;
    Some(parsed)
}
    
```

***




### Explicit Importing

``` st
std
  ::result
         ::Result
  ::collections
              ::HashMap
              ::Vec
              ::BTreeSet

use std::result::Result;
use std::collections::{HashMap, Vec};
```

--- :: master: code

``` st
src/
  lib.rs
  utils.rs
  project.rs
```

``` rs
mod utils;
mod project;

use std::result::Result;
```

--- :: master: code, layout: left

## Unit Tests

``` rust
fn my_func(u: usize) -> bool { ... }

// At the bottom of source file
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_func() {
      assert_eq!(my_func(3), true);
    }
}

```

--- :: master: code



# Traits / Protocols

## Interfaces

--- :: master: regular1




:: label: Swift
``` swift

protocol MyProtocol {
    func partyBoy() -> Int;
}

extension Hello: MyProtocol {
    func partyBoy() -> Int {
        42
    }
}
        
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

trait MyTrait {
    fn party_boy() -> i32;
}

impl MyTrait for Hello {
    func party_boy() -> i32 {
        42
    }
}
        
```

*** :: alignment: left



:: label: Swift
``` swift

protocol MyProtocol {
    associatedType Output;
    func partyBoy() -> Self.Output;
}

extension Hello: MyProtocol {
    func partyBoy() -> Int {
        42
    }
}
        
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

trait MyTrait: PartialEq {
    type Output;
    fn party_boy(&self) -> Self::Output;
}

impl MyTrait for Hello {
    type Output = i32;
    fn party_boy(&self) -> i32 {
        42
    }
}
        
```

*** :: alignment: left



:: label: Swift
``` swift

struct Hello {
    let first: String
    var second: Int
}

extension Hello: Equatable {
    static func == (lhs: Self, rhs: Self) -> Bool {
        lhs.second == rhs.second
    }
}
        
```

:: label: Rust, animate: (0.50;move;bottom)
``` rust

struct Hello {
    first: String,
    second: i32
}

impl PartialEq for Hello {
    fn eq(&self, other: &Self)
      -> bool {
        self.second == other.second
    }
}
        
```

*** :: alignment: left



:: label: Rust
``` rust

// std::iter::Iterator Trait
pub trait Iterator {
    ...
    fn count(self) -> usize
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    fn take_while<P>(self, predicate: P) -> TakeWhile<Self, P>
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, U, F>
    fn enumerate(self) -> Enumerate<Self>
    ...
}
        
```

***

:: label: Rust, animate: (0.50;move;bottom)
``` rust

data.iter()
    .map(|a| a * 2)
    .filter(|a| a > 10)
    .take(3)
    .sum();
        
```

*** :: alignment: left



:: label: Rust
``` rust

// Getting a iterator
collection.iter().map(...)

        
```

***




# Ownership

--- :: master: title1

``` rust

let container = vec![1, 2, 3];
let other_container = container;

println!("{}", other_container);

// Error: borrow of moved value: `container`
println!("{}", container);

```

--- :: master: code

``` rust

let container = vec![1, 2, 3];

// Don't move, *borrow*
let other_container = &container;

println!("{}", &other_container);

println!("{}", &container);

```

--- :: master: code

# `&something` means **borrow** something

## Don't own it, just borrow it, somebody else still owns it.

--- :: master: regular2

``` rust
fn ppp(value: &Vec<usize>) {
    println!("{value}");
}

let x = vec![1, 2, 3];
ppp(&x);

// Works!
println!("{:?}", &x);

```

--- :: master: code

## Some types change shape as references [^1]

:: size: 80
| Owned  | Borrowed |
|-------|-------|
| String | &str  |
| Vec<T> | &[T]  |
| PathBuf | &Path   |

[^1]: `&String` or `&Vec<T>` is not wrong, but will emit a warning

--- :: master: regular1

``` rust
let first: &str = "Hello";
let second: String = "Hello".to_owned();

fn test(input: &str) -> { ... }

test(first);
test(second); // Fails
test(&second); // works
```

--- :: master: code

## Because

- `String` is just a `&str` allocated on the heap
- `Vec<T>` is just a `&[T]` allocated on the heap

--- :: master: regular2

# Second Task

--- :: master: title1

## Tip: Unwrap

- Instead of using `match` or `if let` you can use `.unwrap()` which force unwraps (like `!` in Swift)

```
let x: Option<usize> = None;

use_me(x.unwrap());
```

--- :: master: regular2

## Tip: Clone

> `error[E0382]: borrow of moved value: name-of-variable`

Try `.clone()`.

```
let u = MyStruct::default();
my_func(u.clone());
```

--- :: master: regular2

# Project 2

- Detect language of Tweet
- Remove stop words depending on the tweet language
- Find most used words in a set of tweets
- Some code has already been written
- Checkout `utils.rs`.
- `Readme.md`

***

# Demo

- `cargo run --release`
- Docs
- Crates.io


--- :: master: regular2

## Project 2

- Read `Readme.md` to learn more
- Total: 20min
- After 15min I will share the password for the solution so you can compare your solution if you want
- Use `cargo doc --open`

---

```
final_words = []
words = tweet.split
stop_words = stop_words_for_language(language)
for word in words:
  if word in stop_words:continue
  If word.begins_with("#"):continue
  final_words.append(word)
```

--- :: master: regular1



# Building Mobile Libraries

--- :: master: title1

## Uniffi

- Mozilla Foundation Project
- Takes Rust code, generates libraries for different platforms
- Kotlin, Swift, Go, C#

--- :: master: regular1

``` rs
pub struct EmojiCount {
  pub emoji: String,
  pub count: u32
}

pub fn next_items(limit: u32) -> Vec<EmojiCount> { .. }
```

***

``` udl
dictionary EmojiCount {
    string emoji;
    u32 count;
};

interface EmojiStream {
    sequence<EmojiCount> next_items(u32 limit);
};

```

--- 



## Cargo Swift

- Builds on Uniffi
- Generates Swift Packages

***

# Demo

--- :: master: title1



# Outlook


--- :: master: regular1





## Macros in Rust

- Build-In Macro system
- Expand the language at compile time

``` rust
format!("{}", hello);
```



--- :: master: regular1





## Macros in Rust

- Build-In Macro system
- Expand the language at compile time

``` rust
html! {
        <body>
            <h1>{ content.title }</h1>
            { content.items.map(|item| html!(
                <h2>{ format!("{}", item.name) }</h2>
            }
        </body>
    }
```



--- :: master: regular1





## Lots of crazy stuff

``` rust
let n = 5;
python! {
    for i in range('n):
        print(i, "Hello", 'who)
    print("Goodbye")
}
```



--- :: master: regular1




## Rayon

One additional line to run your code on all cores

``` rust
data
    .par_iter()
    .map(|a| a * 2)
    .filter(|a| a > 10)
    .take(3)
    .sum();
```

--- :: master: code

## Other Targets

- C: cbindgen
- C++: cxx
- Swift: swift-bridge
- Flutter: flutter_rust_bridge
- Dart: membrane
- Python: Pyo3
- Ruby: rutie
- Javascript / Wasm: Trunk
- Example: https://github.com/imWildCat/uniffi-rs-fullstack-examples

---

# Why not Rust?





*** :: master: title1

- Complex Language
- You add a completely different build system to your probably already complex build system
- Most apps just display JSON with enhancements
- Sometimes, having two implementations of the same algorithm is beneficial
- On iOS, Bitcode integration is a tad tricky
- On iOS, LLVM-Rust and LLVM-Xcode are sometimes out of sync, so you have to defer updating Rust or Xcode until they're in sync again


***

# Workshops

- I have longer, more detailed versions of this workshop available
- If your company considers using Rust for cross platform code. Feel free to ping me
- I've done this successfully with 3 apps on the App Store now (for different companies)

