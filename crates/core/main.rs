mod app;
mod utils;

use core::panic;
use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use jg::logic::search_values;
use jg::matcher::compile_regex;
use rayon::prelude::*;
use serde_json::Value;
use walkdir::WalkDir;

pub fn main() {
    let validated = match app::validate(app::cli()) {
        Ok(n) => n,
        Err(e) => {
            panic!(&e);
        }
    };

    let matcher =
        Arc::new(compile_regex(&serde_json::from_str(&validated.regex.as_str()).unwrap()).unwrap());

    let write_lock = Arc::new(Mutex::new(()));

    if !validated.is_dir {
        let (i, mut o) = utils::open_single(&validated);

        let j: Value = serde_json::from_reader(i).unwrap();
        write!(
            o,
            "{}",
            serde_json::to_string_pretty(&search_values(&j, matcher.clone()).unwrap()).unwrap()
        )
        .unwrap();
    } else {
        let file_list = WalkDir::new(validated.input.unwrap())
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|x| x.unwrap().into_path())
            .filter(|x| {
                if let Some(n) = x.extension() {
                    n.to_str().unwrap().to_uppercase() == "JSON"
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        file_list
            .par_iter()
            .map(|x| {
                (
                    x,
                    serde_json::from_reader::<_, Value>(File::open(x).unwrap()),
                )
            })
            .filter(|x| x.1.is_ok())
            .map(|(p, v)| (p, v.unwrap()))
            .for_each(|(p, v)| {
                let results = search_values(&v, matcher.clone()).unwrap();
                if results.is_empty() {
                    return;
                }
                let arc = write_lock.clone();
                let _lock = arc.lock().unwrap();
                println!("{}", p.display());
                println!("{}", serde_json::to_string_pretty(&results).unwrap());
                println!("\\\\");
            });
    }
}
