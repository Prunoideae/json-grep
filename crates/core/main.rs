mod app;
mod utils;

use core::panic;
use std::{fs::File, sync::Arc};

use app::validate;
use jg::logic::search_values;
use jg::matcher::compile_regex;
use serde_json::Value;

pub fn main() {
    let validated = match app::validate(app::cli()) {
        Ok(n) => n,
        Err(e) => {
            panic!(&e);
        }
    };

    if !validated.is_dir {
        let (i, mut o) = utils::open_single(&validated);
        let matcher = Arc::new(
            compile_regex(&serde_json::from_str(&validated.regex.as_str()).unwrap()).unwrap(),
        );

        let j: Value = serde_json::from_reader(i).unwrap();
        write!(
            o,
            "{}",
            serde_json::to_string_pretty(&search_values(&j, matcher.clone()).unwrap()).unwrap()
        )
        .unwrap();
    }
}
