use std::collections::HashMap;

use crate::utils::{self, AppendContents, EasyTake, FindMore, Tweet};
use color_eyre::Result;
use stop_words::{get, LANGUAGE};
use whatlang::detect_lang;

pub fn parse(filename: String) -> Result<HashMap<String, usize>> {
    // During slides: Mention that in Rust almost everything is throwing. so many even simple
    // operations can fail, rust will alert you to this (such as getting a string from a path)
    let data = std::fs::read(filename)?;

    // Rust analyzer tells us quick fix is to add a reference here. explain that
    let tweets: Vec<Tweet> = serde_json::from_slice(&data)?;
    let words: Vec<String> = parse_tweets(tweets);

    Ok(group_words(words))
}

/// Parse all tweets, remove the stop words and useless words
/// join the contents into one big Vec<String> of words
fn parse_tweets(tweets: Vec<Tweet>) -> Vec<String> {
    unimplemented!()
}

/// Parse a tweet into the individual words using `parse_tweet`
fn parse_tweet(tweet: Tweet) -> Option<Vec<String>> {
    let lang = detect_lang(&tweet.text)?;
    let language = utils::convert(lang)?;
    let words = parse_text(tweet.text, language);
    Some(words)
}

/// Split the text into a Vec of words,
/// Filter out the stop words for `LANGUAGE`
/// filter out twitter_words
/// use `is_number` and `is_ascii`
fn parse_text(text: String, language: LANGUAGE) -> Vec<String> {
    // Provided for convenience for the advanced task
    let twitter_words = vec![
        ":-", ")", "=", "PZ", "*", "-", "+", "||", ":", "RT", "Follow", ".", ",", "-", "?",
        "Retweet",
    ];

    unimplemented!()
}

// Group words. Use the hashmap entry api
// https://exercism.org/tracks/rust/concepts/entry-api
fn group_words(words: Vec<String>) -> HashMap<String, usize> {
    unimplemented!()
}

/// Advanced Task
/// Return true if the word contains just numbers
fn is_number(word: &str) -> bool {
    word.chars().all(|c| c.is_numeric())
}

/// Return true if the word contains just ascii
fn is_ascii(word: &str) -> bool {
    word.chars().all(|c| c.is_ascii())
}

/// Advanced Task
/// Find the top `count` words in a HashMap based on the `Value`.
/// Return in sorted order
pub fn top_words(input: HashMap<String, usize>, count: usize) -> Vec<(String, usize)> {
    vec![]
}

// Make a proper test, and keep the test!
#[cfg(test)]
mod tests {
    use super::*;

    /// Tes that 'parse_text' works correctly and removes stop words correctly
    #[test]
    fn test_parse_text() {
        let text = "Hast du meinen Hut gesehen";
        let output = parse_text(text.to_string(), LANGUAGE::German);
        assert_eq!(&output, &["Hut".to_string(), "gesehen".to_string()]);
    }

    /// Test that parse_tweets parses tweets correctly into individual non-stopwords
    #[test]
    fn test_parse_tweets() {
        let tweets = vec![
            Tweet {
                id: 0,
                text: "Hello World".to_owned(),
            },
            Tweet {
                id: 0,
                text: "Beautiful World".to_owned(),
            },
            Tweet {
                id: 0,
                text: "1234 + - RT".to_owned(),
            },
        ];

        let words = parse_tweets(tweets);
        assert!(words.contains_from("Hello"));
        assert!(words.contains_from("World"));
        assert!(words.contains_from("Beautiful"));
        assert!(!words.contains_from("1234"));
        assert!(!words.contains_from("+"));
        assert!(!words.contains_from("-"));
        assert!(!words.contains_from("RT"));
    }

    /// Test that top_words sorts correctly and limits correctly
    #[test]
    fn test_topwords() {
        let mut words = HashMap::new();
        words.insert("second".to_string(), 200);
        words.insert("third".to_string(), 100);
        words.insert("first".to_string(), 300);
        let top = top_words(words, 2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].1, 300);
    }
}
