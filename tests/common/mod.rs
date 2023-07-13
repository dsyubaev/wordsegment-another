use lazy_static::lazy_static;
use wordsegment_another::corpus_loader::Corpus;

lazy_static! {
    static ref GLOBAL_CORPUS: Corpus = Corpus::new("./data/unigrams.txt", "./data/bigrams.txt",);
}

pub fn load() -> &'static Corpus {
    &*GLOBAL_CORPUS
}
