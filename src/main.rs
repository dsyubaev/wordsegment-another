extern crate env_logger;
use std::time::Instant;

use log::{info, SetLoggerError};
use wordsegment_another;

fn main() -> Result<(), SetLoggerError> {
    env_logger::builder().format_timestamp_millis().init();

    info!("Create Segmentator");
    let start = Instant::now();

    let mut seg = wordsegment_another::Segmentator::new("unigrams.txt", "bigrams.txt", "words.txt");

    let duration = start.elapsed();
    info!("Done Segmentator duration {:?}", duration);

    let a = seg.words[0..3].to_vec();
    info!("{:?}", a);

    let x = seg.score("the", Some("in"));
    info!("in the score {:?}", x);

    let x = seg.score("the", None);
    info!("Done scoreing {:?}", x);

    let x = seg.segment("thes");
    info!("Done isegment {:?}", x);

    Ok(())
}
