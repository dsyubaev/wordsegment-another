extern crate env_logger;
use std::time::Instant;

use log::{info, SetLoggerError};
use wordsegment_another;

fn main() -> Result<(), SetLoggerError> {
    env_logger::builder().format_timestamp_millis().init();

    info!("Create Segmentator");
    let start = Instant::now();

    let seg = wordsegment_another::Segmentator::new(
        "./data/unigrams.txt",
        "./data/bigrams.txt",
        "./data/words.txt",
    );

    let duration = start.elapsed();
    info!("Done Segmentator duration {:?}", duration);

    let a = seg.words[0..3].to_vec();
    info!("{:?}", a);

    let x = seg.score("the", Some("in"));
    info!("the score {:?}", x);

    let x = seg.score("the", None);
    info!("Done scoreing {:?}", x);

    let x = seg.segment("choosespain");
    info!("Done segment {:?}", x);

    Ok(())
}
