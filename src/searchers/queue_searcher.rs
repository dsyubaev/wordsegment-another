use crate::corpus::Corpus;
use crate::searchers;
use crate::searchers::{Node, LIMIT, TOTAL};
use log::debug;
use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct QueueSearcher<'a> {
    corpus: &'a Corpus,
}
impl<'a> QueueSearcher<'a> {
    pub fn new(corpus: &'a Corpus) -> QueueSearcher<'a> {
        QueueSearcher { corpus: &corpus }
    }

    fn score_from_string(&self, word: String, previous: Option<String>) -> f64 {
        match previous {
            None => self.score(word.as_str(), None),
            Some(val) => self.score(word.as_str(), Some(val.as_str())),
        }
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

impl<'a> searchers::Searcher for QueueSearcher<'a> {
    fn search(
        &self,
        memo: &mut HashMap<Node, (f64, Vec<String>)>,
        text_: &str,
        previous_: Option<&str>,
    ) -> (f64, Vec<String>) {
        let text = text_.to_string();
        let previous = match previous_ {
            Some(s) => Some(s.to_string()),
            None => None,
        };
        let mut queue: VecDeque<Node> = VecDeque::new();
        let mut is_seen: HashSet<Node> = HashSet::new();
        let init_node = Node {
            text: text.to_owned(),
            previous: previous.to_owned(),
        };
        queue.push_front(init_node.to_owned());

        while !queue.is_empty() {
            //debug!("queue.pop_front = {}")
            let node = queue.pop_front().unwrap();
            // Node is visited and we know it's score. Skip further computation
            if *&memo.contains_key(&node) {
                continue;
            }
            if node.text.is_empty() {
                let res: Vec<String> = Vec::new();
                let _ = &memo.insert(node.to_owned(), (0_f64, res));
            } else {
                if is_seen.contains(&node) {
                    // Node was visited collect all childs from memo and calculate result
                    let mut childs_score = vec![];
                    let cast_previous = node.previous.to_owned();
                    for (prefix, suffix) in devide_vec(&node.text.to_owned()) {
                        let node_child = Node {
                            text: suffix,
                            previous: Some(prefix.to_owned()),
                        };

                        let (suffix_score, suffix_words) = match &memo.get(&node_child) {
                            Some(val) => val,
                            _ => {
                                debug!(
                                    "pair2={:?},\nqueue={:?}, \nis_seen={:?},\nmemo={:?}",
                                    node_child, queue, is_seen, memo
                                );
                                panic!("You should be here")
                            }
                        };

                        let prefix_score =
                            self.score_from_string(prefix.to_owned(), cast_previous.to_owned());
                        let total_score = suffix_score + prefix_score;

                        let mut words: Vec<String> =
                            Vec::with_capacity(suffix_words.capacity() + 1);
                        words.push(prefix.to_string());
                        for s in suffix_words {
                            words.push(s.to_string());
                        }

                        childs_score.push((total_score, words));
                    }
                    // get_max score for childs
                    let best_split = childs_score
                        .iter()
                        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
                        .unwrap();
                    let _ = &memo.insert(node.to_owned(), best_split.to_owned());
                } else {
                    // Node is not visited. Push it back and it's children to the queue
                    queue.push_front(node.to_owned());

                    for (prefix, suffix) in devide_vec(&node.text) {
                        queue.push_front(Node {
                            text: suffix,
                            previous: Some(prefix),
                        });
                    }
                }
                is_seen.insert(node);
            }
        }
        let res = &memo.get(&init_node).unwrap();
        res.to_owned().to_owned().to_owned()
    }
}

pub fn devide_vec(text: &str) -> Vec<(String, String)> {
    let split_size = min(LIMIT, text.len()) + 1;
    let mut res = vec![];
    for i in 1..split_size {
        let val = (text[0..i].to_string(), text[i..].to_string());
        res.push(val);
    }
    res
}
