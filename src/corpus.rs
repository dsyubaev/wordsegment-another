use lazy_static::lazy_static;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

lazy_static! {
    pub static ref GLOBAL_CORPUS: Corpus =
        Corpus::new("./data/unigrams.txt", "./data/bigrams.txt",);
}

pub fn global_corpus() -> &'static Corpus {
    &*GLOBAL_CORPUS
}

#[pyclass]
pub struct Corpus {
    pub unigrams: HashMap<String, i64>,
    pub bigrams: HashMap<String, i64>,
}

#[pymethods]
impl Corpus {
    #[new]
    #[pyo3(signature = (unigrams_file, bigrams_file))]
    pub fn new(unigrams_file: &str, bigrams_file: &str) -> Self {
        let unigrams = parse(unigrams_file).unwrap();
        let bigrams = parse(bigrams_file).unwrap();
        Self { unigrams, bigrams }
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
