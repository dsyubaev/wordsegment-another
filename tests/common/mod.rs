use lazy_static::lazy_static;
use wordsegment_another::scorer::Scorer;

lazy_static! {
    static ref GLOBAL_SCORER: Scorer = Scorer::new("./data/unigrams.txt", "./data/bigrams.txt",);
}

pub fn load() -> &'static Scorer {
    &*GLOBAL_SCORER
}
