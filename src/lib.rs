use num_bigfloat::BigFloat;
use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use log::debug;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn wordsegment_another(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

const TOTAL: f64 = 1024908267229_f64;
const LIMIT: usize = 24;

pub struct Segmentator {
    pub unigrams: HashMap<String, i64>,
    pub bigrams: HashMap<String, i64>,
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

    pub fn score(&self, word: &str, previous: Option<&str>) -> BigFloat {
        let total = BigFloat::from_f64(TOTAL);
        match previous {
            None => {
                if self.unigrams.contains_key(word) {
                    let unigram_val =
                        BigFloat::from_i64(self.unigrams.get(word).unwrap().to_owned());
                    unigram_val / total
                } else {
                    scorei(word.len() as i32)
                }
            }
            Some(previous) => {
                let bigram = format!("{} {}", previous, word);
                //debug!("bigram={:?}", bigram);
                if self.bigrams.contains_key(bigram.as_str()) & self.unigrams.contains_key(previous)
                {
                    let bigram_val =
                        BigFloat::from_i64(self.bigrams.get(bigram.as_str()).unwrap().to_owned());
                    (bigram_val / total) / self.score(previous, None)
                } else {
                    self.score(word, None)
                }
            }
        }
    }

    pub fn search(
        &self,
        text: &str,
        previous: Option<&str>,
        memo: &mut HashMap<(String, String), (f64, Vec<String>)>,
    ) -> (f64, Vec<String>) {
        let res: Vec<String> = Vec::new();
        if text.is_empty() {
            return (0_f64, res);
        }

        let mut current_max = f64::MIN;
        let mut current_res: Vec<String> = Vec::new();

        let mut words: Vec<String> = Vec::new();
        for (prefix, suffix) in devide(text) {
            debug!("{:?} {:?}", prefix, suffix);

            let prev = match previous {
                None => "<s>",
                Some(val) => val,
            };
            let prefix_score = (self.score(prefix, Some(prev))).log10().to_f64();
            let pair = (suffix.to_string(), prefix.to_string());
            if !&memo.contains_key(&pair) {
                let v = (&self.search(suffix, Some(prefix), memo)).to_owned();
                let _ = &memo.insert(pair.to_owned(), v);
            }
            //dbg!(&self.memo);
            let (suffix_score, suffix_words) = (&memo.get(&pair)).unwrap();
            let total_score = suffix_score + prefix_score;
            words.clear();
            words.push(prefix.to_string());
            words.extend(suffix_words.to_vec().into_iter());

            if total_score > current_max {
                current_max = total_score.to_owned();
                current_res = words.clone();
            }
        }
        debug!(
            "ans max are {:?} {:?} for text {:?} {:?}",
            current_max, current_res, text, previous
        );
        (current_max, current_res)
    }
    // Return iterator of words that is the best segmenation of `text`.
    pub fn segment(&self, text: &str) -> Vec<String> {
        debug!("segment called with text={:?}", text);
        let mut memo: HashMap<(String, String), (f64, Vec<String>)> = HashMap::new();
        let mut res: Vec<String> = Vec::new();

        let clean_text = clean(text);
        let size: usize = 250;
        let mut prefix: String = "".to_string();

        let words_skip_size: usize = 5;

        for offset in (0..clean_text.len()).step_by(size) {
            let chunk = &clean_text[offset..min(clean_text.len(), offset + size)];

            debug!("chunk={:?}", chunk);

            prefix.push_str(chunk);
            let (_, chunk_words) = &self.search(prefix.as_str(), None, &mut memo);
            prefix = join_last_n_words(chunk_words, words_skip_size);

            debug!("prefix={:?}", prefix);
            // copy all except last words_skip_size elements
            insert_to_vec(&chunk_words, &mut res, words_skip_size);
        }
        let (_, prefix_words) = &self.search(prefix.as_str(), None, &mut memo);
        insert_to_vec(&prefix_words, &mut res, 0);
        res
    }
}

/// Yield `(prefix, suffix)` pairs from `text`.
///     def divide(self, text):
///         for pos in range(1, min(len(text), self.limit) + 1):
///             yield (text[:pos], text[pos:])
///
pub fn devide<'a>(text: &'a str) -> impl Iterator<Item = (&str, &str)> {
    let split_size = min(LIMIT, text.len()) + 1;

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

/// Helper function to concat last n elements to String
/// ["a", "b"]
pub fn join_last_n_words(x: &Vec<String>, n: usize) -> String {
    x.iter()
        .rev()
        .take(n)
        .rev()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("")
}

/// Insert to destionation vector elements, except the last of skip_last size
/// # Arguments
///
/// * src - The source vec copy from
/// * dest - The destionation vector copy to
/// * skip_last - Number of elements from tail to skip
pub fn insert_to_vec(src: &Vec<String>, dest: &mut Vec<String>, skip_last: usize) -> () {
    if src.len() > skip_last {
        let max_ind = src.len() - skip_last;
        for (i, el) in src.iter().enumerate() {
            if i >= max_ind {
                break;
            }
            dest.push(el.to_string());
        }
    }
}

pub fn scorei(i: i32) -> BigFloat {
    let total = BigFloat::from_f64(TOTAL);
    let x = BigFloat::from_i32(i);
    BigFloat::from_i32(10) / (total * BigFloat::from_i32(10).pow(&x))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(b, [("a", "b"), ("ab", "")]);

        let b: Vec<(&str, &str)> = devide("abc").collect();
        assert_eq!(b, [("a", "bc"), ("ab", "c"), ("abc", "")])
    }

    #[test]
    fn test_join_last_n_words() {
        let mut v: Vec<String> = Vec::new();
        v.push("a".to_string());
        v.push("b".to_string());
        v.push("c".to_string());
        v.push("d".to_string());
        //dbg!(v);

        let g: String = join_last_n_words(&v, 2);
        assert_eq!("cd", g);

        //let x = vec!["a", "b", "c"];
        //assert_eq!("bc", join_last_n_words(x, 2));
    }
    #[test]
    fn test_copy_vec() {
        let mut v: Vec<String> = Vec::new();
        v.push("a".to_string());
        v.push("b".to_string());
        v.push("c".to_string());
        let mut d: Vec<String> = Vec::new();
        insert_to_vec(&mut v, &mut d, 2);
        assert_eq!(vec!["a",], d);

        let mut d: Vec<String> = Vec::new();
        insert_to_vec(&mut v, &mut d, 3);
        assert_eq!(Vec::<String>::new(), d);

        let mut d: Vec<String> = Vec::new();
        d.push("f".to_string());
        insert_to_vec(&mut v, &mut d, 2);
        assert_eq!(vec!["f", "a",], d);
    }
}
