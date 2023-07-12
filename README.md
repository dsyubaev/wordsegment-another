## About The Project

Another one implementation of [wordsegment](https://github.com/grantjenks/python-wordsegment) rewritten in Rust. The binding uses [pyo3](https://github.com/PyO3/pyo3) to interact with the rust package.


```bash
# run specific test
RUST_LOG=info cargo test --test test_segmentator 'test_segment_time' -- --nocapture

```


```bash
maturin develop -r 
python
>>> import wordsegment_another as ws
>>> seg = ws.Segmentator("./data/unigrams.txt", "./data/bigrams.txt", "./data/words.txt")
>>> s = ["it", "was", "a", "bright", "cold", "day", "in", "april", "and", "the", "clocks", "were",
        "striking", "thirteen"]
>>> seg.segment("".join(s))

```