use crate::searchers::{Node, Searcher};
use log::debug;
//use pyo3::prelude::*;
use std::cmp::min;
use std::collections::HashMap;

pub struct Segmentation<'a> {
    searcher: &'a (dyn Searcher + 'a),
}

impl<'a> Segmentation<'a> {
    pub fn new(s: &'a dyn Searcher) -> Segmentation<'a> {
        Segmentation { searcher: s }
    }
    pub fn segment(&self, text: &str) -> Vec<String> {
        debug!("segment called with text={:?}", text);
        let mut memo: HashMap<Node, (f64, Vec<String>)> = HashMap::new();
        let mut res: Vec<String> = Vec::new();

        let clean_text = clean(text);
        let size: usize = 250;
        let mut prefix: String = "".to_string();

        let words_skip_size: usize = 5;

        for offset in (0..clean_text.len()).step_by(size) {
            let chunk = &clean_text[offset..min(clean_text.len(), offset + size)];

            debug!("chunk={:?}", chunk);

            prefix.push_str(chunk);
            let (_, chunk_words) = self.searcher.search(&mut memo, prefix.as_str(), None);
            prefix = join_last_n_words(&chunk_words, words_skip_size);

            debug!("prefix={:?}", prefix);
            // copy all except last words_skip_size elements
            insert_to_vec(&chunk_words, &mut res, words_skip_size);
        }
        let (_, prefix_words) = self.searcher.search(&mut memo, prefix.as_str(), None);
        insert_to_vec(&prefix_words, &mut res, 0);
        res
    }
}

/// Return `text` lower-cased with non-alphanumeric characters removed.
pub fn clean(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
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
