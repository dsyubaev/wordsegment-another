mod common;

extern crate env_logger;
use log::info;
use std::time::Instant;
use wordsegment_another::searchers::{QueueSearcher, RecursiveSearcher};
use wordsegment_another::segmentator::Segmentation;

fn join(result: Vec<&str>) -> String {
    result
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test_segment_small() {
    // using common code.
    let corpus = common::load();
    let search = RecursiveSearcher::new(&corpus);
    let seg = Segmentation::new(&search);
    //env_logger::builder().format_timestamp_millis().init();
    info!("Start test");

    let result = vec!["it", "was"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_queue() {
    // using common code.
    let corpus = common::load();
    let search = QueueSearcher::new(&corpus);
    let seg = Segmentation::new(&search);
    //env_logger::builder().format_timestamp_millis().init();
    info!("Start test");

    let result = vec!["it", "was"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}
