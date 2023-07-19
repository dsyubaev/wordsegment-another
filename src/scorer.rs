use lazy_static::lazy_static;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const TOTAL: f64 = 1024908267229_f64;

lazy_static! {
    pub static ref GLOBAL_SCORER: Scorer =
        Scorer::new("./data/unigrams.txt", "./data/bigrams.txt",);
}

pub fn global_scorer() -> &'static Scorer {
    &*GLOBAL_SCORER
}

#[pyclass]
pub struct Scorer {
    unigrams: HashMap<String, i64>,
    bigrams: HashMap<String, i64>,
}

#[pymethods]
impl Scorer {
    #[new]
    #[pyo3(signature = (unigrams_file, bigrams_file))]
    pub fn new(unigrams_file: &str, bigrams_file: &str) -> Self {
        let unigrams = parse(unigrams_file).unwrap();
        let bigrams = parse(bigrams_file).unwrap();
        Self { unigrams, bigrams }
    }
}

impl Scorer {
    pub fn score(&self, word: &str, previous: Option<&str>) -> f64 {
        match previous {
            None => {
                if let Some(unigrms_cnt) = self.unigrams.get(word) {
                    (*unigrms_cnt as f64 / TOTAL).log10()
                } else {
                    let word_len = word.len() as f64;
                    // log10 (10 / (total * 10 ** word_len ))
                    1_f64 - (TOTAL.log10() + word_len)
                }
            }
            Some(previous) => {
                let bigram = format!("{} {}", previous, word);
                if let (Some(bigram_cnt), Some(previus_cnt)) = (
                    self.bigrams.get(bigram.as_str()),
                    self.unigrams.get(previous),
                ) {
                    (*bigram_cnt as f64 / *previus_cnt as f64).log10()
                } else {
                    self.score(word, None)
                }
            }
        }
    }
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
