use std::{
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_input_to_iter(p: impl AsRef<Path>) -> impl Iterator<Item = String> {
    stream_lines(BufReader::new(std::fs::File::open(p).unwrap()))
}

pub fn stream_lines(stream: impl BufRead) -> impl Iterator<Item = String> {
    stream.lines().map(Result::unwrap)
}
