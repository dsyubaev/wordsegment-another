mod common;

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
