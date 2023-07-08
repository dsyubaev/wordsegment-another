mod common;

#[test]
fn test_add() {
    // using common code.
    let seg = common::segmentator();
    let _ = seg.segment("thes");
}
