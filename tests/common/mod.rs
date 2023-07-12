use lazy_static::lazy_static;
use wordsegment_another::Segmentator;

lazy_static! {
    static ref GLOBAL_SEGMENTATOR: Segmentator =
        Segmentator::new("./data/unigrams.txt", "./data/bigrams.txt",);
}

pub fn segmentator() -> &'static Segmentator {
    &*GLOBAL_SEGMENTATOR
}
