use crate::corpus::Corpus;
use crate::searchers;
use crate::searchers::{Node, LIMIT, TOTAL};
use log::debug;
use std::cmp::min;
use std::collections::HashMap;

pub struct RecursiveSearcher<'a> {
    corpus: &'a Corpus,
}

impl<'a> RecursiveSearcher<'a> {
    pub fn new(corpus: &'a Corpus) -> RecursiveSearcher<'a> {
        RecursiveSearcher { corpus: &corpus }
    }

    fn score(&self, word: &str, previous: Option<&str>) -> f64 {
        let corpus = &self.corpus;
        match previous {
            None => {
                if let Some(unigrms_cnt) = corpus.unigrams.get(word) {
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
                    corpus.bigrams.get(bigram.as_str()),
                    corpus.unigrams.get(previous),
                ) {
                    (*bigram_cnt as f64 / *previus_cnt as f64).log10()
                } else {
                    self.score(word, None)
                }
            }
        }
    }
}
impl<'a> searchers::Searcher for RecursiveSearcher<'a> {
    fn search(
        &self,
        memo: &mut HashMap<Node, (f64, Vec<String>)>,
        text: &str,
        previous: Option<&str>,
    ) -> (f64, Vec<String>) {
        if text.is_empty() {
            let res: Vec<String> = Vec::new();
            (0_f64, res)
        } else {
            let mut current_max = f64::MIN;
            let mut current_res: Vec<String> = Vec::new();

            for (prefix, suffix) in devide(text) {
                debug!("{:?} {:?}", prefix, suffix);

                let prefix_score = self.score(prefix, previous);
                let pair = Node {
                    text: suffix.to_string(),
                    previous: Some(prefix.to_string()),
                };

                let (suffix_score, suffix_words) = match &memo.get(&pair) {
                    Some((suffix_score, suffix_words)) => {
                        (suffix_score.to_owned(), suffix_words.to_owned())
                    }
                    None => {
                        let (score, words) = (self.search(memo, suffix, Some(prefix))).to_owned();
                        let _ = &memo.insert(pair.to_owned(), (score.to_owned(), words.to_owned()));
                        (score, words)
                    }
                };

                let total_score = suffix_score + prefix_score;

                if total_score > current_max {
                    current_max = total_score.to_owned();
                    let mut words: Vec<String> = Vec::with_capacity(suffix_words.capacity() + 1);
                    words.push(prefix.to_string());
                    words.extend(suffix_words.into_iter());

                    current_res = words.clone();
                }
            }
            debug!(
                "ans max are {:?} {:?} for text {:?} {:?}",
                current_max, current_res, text, previous
            );
            (current_max, current_res)
        }
    }
}

/// Yield `(prefix, suffix)` pairs from `text`.
///     def divide(self, text):
///
///         for pos in range(1, min(len(text), self.limit) + 1):
///             yield (text[:pos], text[pos:])
pub fn devide<'a>(text: &'a str) -> impl Iterator<Item = (&str, &str)> {
    let split_size = min(LIMIT, text.len()) + 1;

    (1..split_size).map(|i| (&text[0..i], &text[i..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devide() {
        let b: Vec<(&str, &str)> = devide("ab").collect();
        assert_eq!(b, [("a", "b"), ("ab", "")]);

        let b: Vec<(&str, &str)> = devide("abc").collect();
        assert_eq!(b, [("a", "bc"), ("ab", "c"), ("abc", "")])
    }
}
