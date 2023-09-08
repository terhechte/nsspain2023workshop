# Project 2

## Task

- Read the contents.json into a `Vec<Tweet>`. Fill out the functions in `project.rs` so that:
- We can see the most used words in all tweets,
- But we filter out useless stop words per language
- And other useless words
- The words are then grouped and sorted.
- You can easily run via `F5`

## Notes

- Checkout the contents of `utils.rs`. They're provided for convenience to simplify some concepts of the language that we haven't introduced yet.
- Write your code in `project.rs`

## Hints

- [Have a look at the `HashMap` `Entry` API](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)
- [Another good `Entry` description](https://exercism.org/tracks/rust/concepts/entry-api)
- In order to use the `HashMap` you have to first import it: `use std::collections::HashMap;`
- [Whatlang Documentation](https://docs.rs/whatlang/latest/whatlang/)
- [Stopwords Documentation](https://docs.rs/stop-words/0.6.2/stop_words/)
- [Error Propagation](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)
- [The Vec type](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html)

## Advanced Task

- Filter out all words containing unicode / emoji characters
- Filter out all #hashtags and @mentions
- Filter out standard twitter words such as `RT`, `Follow`

## Advanced Hints

- [Have a look at the `char` type](https://doc.rust-lang.org/std/primitive.char.html#)
- [Have a look at the `String` type](https://doc.rust-lang.org/std/string/struct.String.html)
