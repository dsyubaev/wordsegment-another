pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::iter::Map;
use std::ops::Range;

const TOTAL: f64 = 1024908267229_f64;
const LIMIT: usize = 24;

pub struct Segmentator {
    unigrams: HashMap<String, i64>,
    bigrams: HashMap<String, i64>,
    pub words: Vec<String>,
}

impl Segmentator {
    pub fn new(unigrams_file: &str, bigrams_file: &str, words_file: &str) -> Self {
        let unigrams = parse(unigrams_file).unwrap();
        let bigrams = parse(bigrams_file).unwrap();
        let words = parse_words(words_file).unwrap();
        Self {
            unigrams,
            bigrams,
            words,
        }
    }

    /// Score `word` in the context of `previous` word.
    pub fn score(&self, word: &str, previous: Option<&str>) -> f64 {
        match previous {
            None => {
                if self.unigrams.contains_key(word) {
                    self.unigrams.get(word).unwrap().to_owned() as f64 / TOTAL
                } else {
                    let base: i32 = 10;
                    10_f64 / (TOTAL * (base.pow(word.len() as u32) as f64))
                }
            }
            Some(previous) => {
                let bigram = format!("{} {}", previous, word);
                if self.bigrams.contains_key(bigram.as_str()) & self.unigrams.contains_key(previous)
                {
                    (self.bigrams.get(bigram.as_str()).unwrap().to_owned() as f64 / TOTAL)
                        / self.score(previous, None)
                } else {
                    self.score(word, None)
                }
            }
        }
    }

    // Return iterator of words that is the best segmenation of `text`.
    pub fn isegment(&self, text: &str) -> Vec<String> {
        let res: Vec<String> = Vec::new();
        res
    }
}

/// Yield `(prefix, suffix)` pairs from `text`.
///     def divide(self, text):
///         for pos in range(1, min(len(text), self.limit) + 1):
///             yield (text[:pos], text[pos:])
///
pub fn devide<'a>(text: &'a str) -> impl Iterator<Item = (&str, &str)> {
    let split_size = cmp::min(LIMIT, text.len());

    (1..split_size).map(|i| (&text[0..i], &text[i..]))
}

/// Return `text` lower-cased with non-alphanumeric characters removed.
pub fn clean(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

pub fn parse_words(path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut words: Vec<String> = Vec::new();
    for line in reader.lines() {
        let w = line.unwrap();
        words.push(w)
    }
    Ok(words)
}

pub fn parse(path: &str) -> Result<HashMap<String, i64>, Error> {
    let mut map = HashMap::new();

    let file = File::open(path)?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    for line in reader.lines() {
        //println!("{}", line?);
        let x = line.unwrap();
        let mut parts = x.split("\t");
        let key = parts.next().unwrap();
        let val = parts.next().unwrap();
        map.insert(key.to_string(), val.parse::<i64>().unwrap());
    }
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_clean() {
        assert_eq!(clean("Today is the 14th of July"), "todayisthethofjuly");

        assert_eq!(clean("  "), "");

        assert_eq!(clean("a14_b"), "ab");

        assert_eq!(clean("Can't buy me love!"), "cantbuymelove")
    }

    #[test]
    fn test_devide() {
        let b: Vec<(&str, &str)> = devide("ab").collect();
        assert_eq!(b, [("a", "b")]);

        let b: Vec<(&str, &str)> = devide("abc").collect();
        assert_eq!(b, [("a", "bc"), ("ab", "c")])
    }
}
