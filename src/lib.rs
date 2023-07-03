pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub fn parse(path: &str) -> Result<HashMap<String, i64>, Error> {
    let mut book_reviews = HashMap::new();

    let file = File::open(path)?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    let mut n = 0;

    for line in reader.lines() {
        //println!("{}", line?);
        let x = line.unwrap();
        let mut parts = x.split("\t");
        let key = parts.next().unwrap();
        let val = parts.next().unwrap();
        book_reviews.insert(key.to_string(), val.parse::<i64>().unwrap());
    }
    Ok(book_reviews)
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
