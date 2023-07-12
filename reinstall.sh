#!/bin/bash
pip uninstall -y -q -q wordsegment_another
rm -f rust/target/wheels/*
maturin build --release

WHEEL=$(ls target/wheels)
pip install target/wheels/$WHEEL