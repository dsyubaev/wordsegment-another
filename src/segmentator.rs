use crate::corpus_loader::{global_corpus, Corpus};
use log::debug;
use pyo3::prelude::*;
use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};

const TOTAL: f64 = 1024908267229_f64;
const LIMIT: usize = 24;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Node {
    pub text: String,
    pub previous: Option<String>,
}

#[pyfunction]
pub fn segment(text: &str) -> Vec<String> {
    let selector = 1;
    let search_slected = match selector {
        0 => search,
        _ => search_non_rec,
    };

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
        let (_, chunk_words) = search_slected(&mut memo, prefix.as_str(), None);
        prefix = join_last_n_words(&chunk_words, words_skip_size);

        debug!("prefix={:?}", prefix);
        // copy all except last words_skip_size elements
        insert_to_vec(&chunk_words, &mut res, words_skip_size);
    }
    let (_, prefix_words) = search_slected(&mut memo, prefix.as_str(), None);
    insert_to_vec(&prefix_words, &mut res, 0);
    res
}

pub fn score_from_string(word: String, previous: Option<String>) -> f64 {
    match previous {
        None => score(word.as_str(), None),
        Some(val) => score(word.as_str(), Some(val.as_str())),
    }
}

pub fn score(word: &str, previous: Option<&str>) -> f64 {
    let corpus = global_corpus();
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
                score(word, None)
            }
        }
    }
}

fn search(
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

            let prefix_score = score(prefix, previous);
            let pair = Node {
                text: suffix.to_string(),
                previous: Some(prefix.to_string()),
            };

            let (suffix_score, suffix_words) = match &memo.get(&pair) {
                Some((suffix_score, suffix_words)) => {
                    (suffix_score.to_owned(), suffix_words.to_owned())
                }
                None => {
                    let (score, words) = (search(memo, suffix, Some(prefix))).to_owned();
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

/// Тот же поиск здесь 2 раза приходится делать обход потомков узла
/// 1 ый раз что бы понять кого посчитать
/// 2ой раз что бы собрать посчитанные значения
fn search_non_rec(
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
        if *&memo.contains_key(&node) {
            continue
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
                        score_from_string(prefix.to_owned(), cast_previous.to_owned());
                    let total_score = suffix_score + prefix_score;

                    let mut words: Vec<String> = Vec::with_capacity(suffix_words.capacity() + 1);
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

/// Yield `(prefix, suffix)` pairs from `text`.
///     def divide(self, text):
///         for pos in range(1, min(len(text), self.limit) + 1):
///             yield (text[:pos], text[pos:])
///
pub fn devide<'a>(text: &'a str) -> impl Iterator<Item = (&str, &str)> {
    let split_size = min(LIMIT, text.len()) + 1;

    (1..split_size).map(|i| (&text[0..i], &text[i..]))
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
