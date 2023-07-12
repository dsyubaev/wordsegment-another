import timeit
import wordsegment_another as ws_another
from wordsegment import load, segment

seg = ws_another.Segmentator("./data/unigrams.txt", "./data/bigrams.txt")
load()

words = [
    "it",
    "was",
    "a",
    "bright",
    "cold",
    "day",
    "in",
    "april",
    "and",
    "the",
    "clocks",
    "were",
    "striking",
    "thirteen",
]
words_concat = "".join(words)

x = timeit.timeit(
    "seg.segment(words_concat)",
    number=10,
    setup="from __main__ import seg, words_concat",
)
print(f"wordsegment_another {x:.2f} sec")


x = timeit.timeit(
    "segment(words_concat)",
    number=10,
    setup="from __main__ import segment, words_concat",
)
print(f"wordsegment {x:.2f} sec")
