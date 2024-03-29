![Build status](https://github.com/lhayhurst/rust-prog-lang-2021/actions/workflows/rust.yml/badge.svg)

# rust-prog-lang-2021
Implementing exercises and ideas from the [Rust Programming Language book (2021 edition)](https://doc.rust-lang.org/stable/book/).

## Chpt 3
* [temperature conversion](chpt3/temp_conversion/src/lib.rs): uses the [rstest crate](https://docs.rs/rstest/latest/rstest/) to do pytest style test input parameterization, and the [float_cmp crate](https://docs.rs/float-cmp/latest/float_cmp/) for floating point comparison.
* [nth fibonacci](chpt3/fibonacci/src/lib.rs): also uses `rstest` 

## Chpt 4
* [ownership and references](chpt4/ownership/src/lib.rs)
* [slices](chpt4/slices/src/lib.rs)

## Chpt 5
[Various tests](chpt5/structs/src/lib.rs) related to declaring and initializing structs, adding traits to structs, adding methods, and associated functions.

## Chpt 6
* [enums](chpt6/enums/src/lib.rs)
* [matching](chpt6/patterns/src/lib.rs)

## Chpt 8
* [vectors](chpt8/vec/src/lib.rs)
* [strings](chpt8/string/src/lib.rs)
* [hashmaps](chpt8/hashmap/src/lib.rs)
* [problem1](chpt8/prob1/src/lib.rs): implementation and tests for calculating median and mode for a vector of ints.

# Advent of Code 2023
Mooching of this project to do the [Advent of Code 2023](https://adventofcode.com/2023/); also making generous use of ChatGPT as a co-pilot to see how it feels to use a LLM code pal in a focused way. 

See [the local README](./aoc2023/README.md) for notes. 

