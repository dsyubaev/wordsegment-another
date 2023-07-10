## About The Project

Another one implementation of [wordsegment](https://github.com/grantjenks/python-wordsegment) rewritten in Rust. The binding uses [pyo3](https://github.com/PyO3/pyo3) to interact with the rust package.


```bash
# run specific test
RUST_LOG=info cargo test --test test_segmentator 'test_segment_81' -- --nocapture
```