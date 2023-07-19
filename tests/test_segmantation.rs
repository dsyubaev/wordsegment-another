mod common;

extern crate env_logger;
use log::info;

use common::join;
use rstest::*;
use std::time::Instant;
use wordsegment_another::searchers::{QueueSearcher, RecursiveSearcher, Searcher};
use wordsegment_another::segmentator::Segmentation;

#[fixture]
pub fn fixture_recursive_searcher() -> RecursiveSearcher<'static> {
    let scorer = common::load();
    RecursiveSearcher::new(scorer)
}

#[fixture]
pub fn fixture_q_searcher() -> QueueSearcher<'static> {
    let scorer = common::load();
    QueueSearcher::new(scorer)
}

#[rstest]
#[case::queue(fixture_q_searcher())]
#[case::recursion(fixture_recursive_searcher())]
fn test_search<T: Searcher>(
    #[case] searcher: T,
    #[values(
    vec!["choose", "spain"], vec!["this", "is", "a", "test"],
    vec!["when", "in", "the", "course", "of", "human", "events", "it", "becomes", "necessary"],
    vec!["who", "represents"],
    vec!["experts", "exchange"],
    vec!["speed", "of", "art"],
    vec!["now", "is", "the", "time", "for", "all", "good"],
    vec!["it", "is", "a", "truth", "universally", "acknowledged"],
    vec!["it", "was", "a", "bright", "cold", "day", "in", "april", "and", "the", "clocks", "were", "striking", "thirteen",],
    vec![ "it", "was", "the", "best", "of", "times", "it", "was", "the", "worst", "of", "times", "it", "was", "the", "age", "of", "wisdom", "it", "was", "the", "age", "of", "foolishness", ],
    vec![ "as", "gregor", "samsa", "awoke", "one", "morning", "from", "uneasy", "dreams", "he", "found", "himself", "transformed", "in", "his", "bed", "into", "a", "gigantic", "insect", ],
    vec![ "in", "a", "hole", "in", "the", "ground", "there", "lived", "a", "hobbit", "not", "a", "nasty", "dirty", "wet", "hole", "filled", "with", "the", "ends", "of", "worms", "and", "an", "oozy", "smell", "nor", "yet", "a", "dry", "bare", "sandy", "hole", "with", "nothing", "in", "it", "to", "sit", "down", "on", "or", "to", "eat", "it", "was", "a", "hobbit", "hole", "and", "that", "means", "comfort", ],
    vec![ "far", "out", "in", "the", "uncharted", "backwaters", "of", "the", "unfashionable", "end", "of", "the", "western", "spiral", "arm", "of", "the", "galaxy", "lies", "a", "small", "un", "regarded", "yellow", "sun", ]
    )]
    result: Vec<&str>,
) {
    let seg = Segmentation::new(&searcher);
    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[rstest]
#[case::queue(fixture_q_searcher())]
#[case::recursion(fixture_recursive_searcher())]
fn test_segment_time<T: Searcher>(#[case] searcher: T) {
    let seg = Segmentation::new(&searcher);
    env_logger::builder().format_timestamp_millis().try_init();
    let mut times = vec![];
    for result in [
        vec!["choose", "spain"],
        vec![
            "far",
            "out",
            "in",
            "the",
            "uncharted",
            "backwaters",
            "of",
            "the",
            "unfashionable",
            "end",
            "of",
            "the",
            "western",
            "spiral",
            "arm",
            "of",
            "the",
            "galaxy",
            "lies",
            "a",
            "small",
            "un",
            "regarded",
            "yellow",
            "sun",
        ],
    ] {
        for _ in 0..10 {
            let start = Instant::now();

            let _ = seg.segment(join(result.clone()).as_str());
            let duration = start.elapsed();
            times.push(duration.as_secs_f32());
        }
    }
    let avg = times.iter().sum::<f32>() / times.len() as f32;
    info!("Run avg={:?} times={:?}", avg, times);
}
