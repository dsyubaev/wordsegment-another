import timeit
import wordsegment_another as wsa
from wordsegment import load, segment

NUMBER_OF_RUNS = 20

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
    "wsa.segment(words_concat)",
    number=NUMBER_OF_RUNS,
    setup="from __main__ import wsa, words_concat",
)
print(f"wordsegment_another {x:.2f} sec")


x = timeit.timeit(
    "segment(words_concat)",
    number=NUMBER_OF_RUNS,
    setup="from __main__ import segment, words_concat",
)
print(f"wordsegment {x:.2f} sec")
