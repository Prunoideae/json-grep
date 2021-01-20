use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use app::SearchConfig;

use crate::app;

pub fn open_single(config: &SearchConfig) -> (Box<dyn BufRead>, Box<dyn Write>) {
    (
        match &config.input {
            Some(n) => Box::new(BufReader::with_capacity(
                1024 * 128,
                File::open(Path::new(n)).unwrap(),
            )),
            None => Box::new(BufReader::with_capacity(1024 * 128, stdin())),
        },
        match &config.output {
            Some(n) => Box::new(BufWriter::with_capacity(
                1024 * 128,
                File::create(Path::new(n)).unwrap(),
            )),
            None => Box::new(BufWriter::with_capacity(1024 * 128, stdout())),
        },
    )
}
