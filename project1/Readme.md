# Project 1

## Task

Read all characters from a file and use a closure to count how many of these symbols are Math symbols.
Math symbols are defined as the numbers 0-9 and the graphical symbols such as `~`, `!`, or `+`: The ascii range U+0021 ‘!’ ..= U+007E ‘~’.

- Fill out the existing `src/main.rs`
- You can easily run via `F5`

## Hints

- [Have a look at the `char` type](https://doc.rust-lang.org/std/primitive.char.html#)
- [Have a look at the `String` type](https://doc.rust-lang.org/std/string/struct.String.html)
- [Closure Documentation](https://doc.rust-lang.org/book/ch13-01-closures.html)

## Advanced Task

- Write the function `count_characters_extra`
- It uses a HashMap to count how often each word appears

``` rs
fn count_characters_extra(
    in_content: String,
    action: impl Fn(char) -> bool,
) -> (usize, HashMap<char, usize>)
```

## Advanced Hints

- [Have a look at the `HashMap` `Entry` API](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)
- [Another good `Entry` description](https://exercism.org/tracks/rust/concepts/entry-api)
- In order to use the `HashMap` you have to first import it: `use std::collections::HashMap;`

## If you have a question, ask me right away! Here, or on Slack
