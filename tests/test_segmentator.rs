mod common;

extern crate env_logger;
use log::info;
use std::time::Instant;

fn join(result: Vec<&str>) -> String {
    result
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test_segment_0() {
    // using common code.
    let seg = common::segmentator();
    let result = vec!["choose", "spain"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_1() {
    // using common code.
    let seg = common::segmentator();
    let result = vec!["this", "is", "a", "test"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_2() {
    // using common code.
    let seg = common::segmentator();
    let result = vec![
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
    ];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_3() {
    // using common code.
    let seg = common::segmentator();
    let result = vec!["who", "represents"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_4() {
    // using common code.
    let seg = common::segmentator();
    let result = vec!["experts", "exchange"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_5() {
    // using common code.
    let seg = common::segmentator();
    let result = vec!["speed", "of", "art"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_6() {
    // using common code.
    let seg = common::segmentator();
    let result = vec!["now", "is", "the", "time", "for", "all", "good"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_7() {
    // using common code.
    let seg = common::segmentator();
    let result = vec!["it", "is", "a", "truth", "universally", "acknowledged"];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_8() {
    // using common code.
    let seg = common::segmentator();
    let result = vec![
        "it", "was", "a", "bright", "cold", "day", "in", "april", "and", "the", "clocks", "were",
        "striking", "thirteen",
    ];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_time() {
    // using common code.
    let seg = common::segmentator();
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

        let _ = seg.segment(join(result.clone()).as_str());
        // assert_eq!(, result);
        let duration = start.elapsed();
        acc += duration.as_secs_f32();
        n += 1;
        info!("Done test duration={:?}", duration);
    }
    info!("Avg time={:?}", acc / n as f32);
}

#[test]
fn test_segment_9() {
    // using common code.
    let seg = common::segmentator();
    let result = vec![
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
    ];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_10() {
    // using common code.
    let seg = common::segmentator();
    let result = vec![
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
    ];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_11() {
    // using common code.
    let seg = common::segmentator();
    let result = vec![
        "in", "a", "hole", "in", "the", "ground", "there", "lived", "a", "hobbit", "not", "a",
        "nasty", "dirty", "wet", "hole", "filled", "with", "the", "ends", "of", "worms", "and",
        "an", "oozy", "smell", "nor", "yet", "a", "dry", "bare", "sandy", "hole", "with",
        "nothing", "in", "it", "to", "sit", "down", "on", "or", "to", "eat", "it", "was", "a",
        "hobbit", "hole", "and", "that", "means", "comfort",
    ];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}

#[test]
fn test_segment_12() {
    // using common code.
    let seg = common::segmentator();
    let result = vec![
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
    ];

    assert_eq!(seg.segment(join(result.clone()).as_str()), result);
}
