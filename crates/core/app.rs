use std::{error::Error, path::Path};

use clap::{App, Arg, ArgMatches};
use jg::matcher::compile_regex;

pub enum Format {
    JSON,
    Plain,
    Auto,
}
pub struct SearchConfig {
    pub pretty: bool,
    pub format: Format,
    pub input: Option<String>,
    pub is_dir: bool,
    pub output: Option<String>,
    pub regex: String,
}

pub fn cli() -> ArgMatches<'static> {
    App::new("json-grep")
        .version("0.1")
        .author("Prunoideae, <2018301050@email.szu.edu.cn>")
        .about("A commandline utility for searching json with regex patterns")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("PATH")
                .required(false)
                .takes_value(true)
                .help("The file or folder to search, leave out to read from stdin."),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("PATH")
                .required(false)
                .takes_value(true)
                .help("The value or folder to write results, leave out to write to stdout."),
        )
        .arg(
            Arg::with_name("regex_file")
                .help("The file contains the regex pattern to search.")
                .required_unless("regex")
                .short("r")
                .long("regex-path")
                .takes_value(true)
                .value_name("PATH"),
        )
        .arg(
            Arg::with_name("regex")
                .help("The regex pattern used to search json.")
                .required_unless("regex_file")
                .index(1),
        )
        .arg(
            Arg::with_name("format")
                .help("The formatting used to write outputs.")
                .long("format")
                .short("f")
                .takes_value(true)
                .possible_values(&["json", "plain"]),
        )
        .arg(
            Arg::with_name("no_pretty")
                .help("Do not prettify the output.")
                .short("p")
                .long("no-pretty")
                .required(false),
        )
        .get_matches()
}

pub fn validate(matches: ArgMatches) -> Result<SearchConfig, String> {
    if matches.is_present("input") {
        let p = Path::new(matches.value_of("input").unwrap());
        if !p.is_file() && !p.is_dir() {
            return Err("Error when validating file.".to_string());
        }
    }

    Ok(SearchConfig {
        regex: match compile_regex(
            match &serde_json::from_str(matches.value_of("regex").unwrap()) {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!(
                        "Error when parsing query \"{}\" as JSON.",
                        matches.value_of("regex").unwrap()
                    ));
                }
            },
        ) {
            Ok(_) => matches.value_of("regex").unwrap().to_string(),
            Err(e) => {
                return Err(e);
            }
        },
        pretty: !matches.is_present("no_pretty"),
        format: if matches.is_present("format") {
            match matches.value_of("format").unwrap() {
                "json" => Format::JSON,
                "plain" => Format::Plain,
                _ => unreachable!(),
            }
        } else {
            Format::Auto
        },
        input: if !matches.is_present("input") {
            None
        } else {
            Some(matches.value_of("input").unwrap().to_string())
        },
        is_dir: matches.is_present("input")
            && Path::new(matches.value_of("input").unwrap()).is_dir(),
        output: if !matches.is_present("output") {
            None
        } else {
            Some(matches.value_of("output").unwrap().to_string())
        },
    })
}
