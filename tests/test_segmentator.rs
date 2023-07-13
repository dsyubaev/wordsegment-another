mod common;

extern crate env_logger;
use log::info;
use std::time::Instant;
use wordsegment_another::segmentator::segment;

fn join(result: Vec<&str>) -> String {
    result
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

// #[test]
// fn test_segment_0() {
//     // using common code.
//     let corpus = common::load();
//     let result = vec!["choose", "spain"];
//
//     assert_eq!(segment(corpus, join(result.clone()).as_str()), result);
// }

macro_rules! test_segment {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let corpus = common::load();
            let result = $value;
            assert_eq!(segment(corpus, join(result.clone()).as_str()), result);

        }
    )*
    }
}
test_segment! {
    test_segment_0: vec!["choose", "spain"],
    test_segment_1: vec!["this", "is", "a", "test"],
    test_segment_2: vec![
        "when",
        "in",
        "the",
        "course",
        "of",
        "human",
        "events",
        "it",
        "becomes",
        "necessary",
    ],
    test_segment_3: vec!["who", "represents"],
    test_segment_4: vec!["experts", "exchange"],
    test_segment_5: vec!["speed", "of", "art"],
    test_segment_6: vec!["now", "is", "the", "time", "for", "all", "good"],
    test_segment_7: vec!["it", "is", "a", "truth", "universally", "acknowledged"],
    test_segment_8: vec![
        "it", "was", "a", "bright", "cold", "day", "in", "april", "and", "the", "clocks", "were",
        "striking", "thirteen",
    ],
    test_segment_9: vec![
        "it",
        "was",
        "the",
        "best",
        "of",
        "times",
        "it",
        "was",
        "the",
        "worst",
        "of",
        "times",
        "it",
        "was",
        "the",
        "age",
        "of",
        "wisdom",
        "it",
        "was",
        "the",
        "age",
        "of",
        "foolishness",
    ],
    test_segment_10: vec![
        "as",
        "gregor",
        "samsa",
        "awoke",
        "one",
        "morning",
        "from",
        "uneasy",
        "dreams",
        "he",
        "found",
        "himself",
        "transformed",
        "in",
        "his",
        "bed",
        "into",
        "a",
        "gigantic",
        "insect",
    ],
    test_segment_11: vec![
        "in", "a", "hole", "in", "the", "ground", "there", "lived", "a", "hobbit", "not", "a",
        "nasty", "dirty", "wet", "hole", "filled", "with", "the", "ends", "of", "worms", "and",
        "an", "oozy", "smell", "nor", "yet", "a", "dry", "bare", "sandy", "hole", "with",
        "nothing", "in", "it", "to", "sit", "down", "on", "or", "to", "eat", "it", "was", "a",
        "hobbit", "hole", "and", "that", "means", "comfort",
    ],
    test_segment_12: vec![
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
}

#[test]
fn test_segment_time() {
    // using common code.
    let corpus = common::load();
    env_logger::builder().format_timestamp_millis().init();
    info!("Done init segmentator");
    let mut acc = 0.0;
    let mut n = 0_u8;
    for _ in 0..5 {
        let start = Instant::now();
        let result = vec![
            "it", "was", "a", "bright", "cold", "day", "in", "april", "and", "the", "clocks",
            "were", "striking", "thirteen",
        ];

        let _ = segment(corpus,join(result.clone()).as_str());
        // assert_eq!(, result);
        let duration = start.elapsed();
        acc += duration.as_secs_f32();
        n += 1;
        info!("Done test duration={:?}", duration);
    }
    info!("Avg time={:?}", acc / n as f32);
}
