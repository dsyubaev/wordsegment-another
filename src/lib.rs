pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub struct Segmentator {
    unigrams: HashMap<String, i64>,
    bigrams: HashMap<String, i64>,
    total: f64,
    limit: i32,
    pub words: Vec<String>,
}

impl Segmentator {
    pub fn new(unigrams_file: &str, bigrams_file: &str, words_file: &str) -> Self {
        let unigrams = parse(unigrams_file).unwrap();
        let bigrams = parse(bigrams_file).unwrap();
        let words = parse_words(words_file).unwrap();
        let total: f64 = 1024908267229_f64;
        let limit: i32 = 24;
        Self {
            unigrams,
            bigrams,
            total,
            limit,
            words,
        }
    }

    /// Score `word` in the context of `previous` word.
    pub fn score(&self, word: &str, previous: Option<&str>) -> f64 {
        match previous {
            None => {
                if self.unigrams.contains_key(word) {
                    self.unigrams.get(word).unwrap().to_owned() as f64 / self.total
                } else {
                    let base: i32 = 10;
                    10_f64 / (self.total * (base.pow(word.len() as u32) as f64))
                }
            }
            Some(previous) => {
                let bigram = format!("{} {}", previous, word);
                if self.bigrams.contains_key(bigram.as_str()) & self.unigrams.contains_key(previous)
                {
                    (self.bigrams.get(bigram.as_str()).unwrap().to_owned() as f64 / self.total)
                        / self.score(previous, None)
                } else {
                    self.score(word, None)
                }
            }
        }
    }
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
}
