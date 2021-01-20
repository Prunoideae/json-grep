use std::{sync::Arc, vec};

use crate::matcher::JsonMatcher;
use rayon::prelude::*;
use serde_json::Value;

struct Configuration {
    pretty: bool,
}
static mut CONFIG: Configuration = Configuration { pretty: false };

pub fn set_pretty(b: bool) {
    unsafe {
        CONFIG.pretty = b;
    }
}

pub fn search_values(
    json: &Value,
    matcher: Arc<Box<dyn JsonMatcher + Send + Sync>>,
) -> Option<Vec<&Value>> {
    match json {
        Value::Null => None,
        Value::Bool(_) => {
            if matcher.jmatch(json) {
                Some(vec![json])
            } else {
                None
            }
        }
        Value::Number(_) => {
            if matcher.jmatch(json) {
                Some(vec![json])
            } else {
                None
            }
        }
        Value::String(_) => {
            if matcher.jmatch(json) {
                Some(vec![json])
            } else {
                None
            }
        }
        Value::Array(v) => {
            if matcher.jmatch(json) {
                Some(vec![json])
            } else {
                Some(
                    v.par_iter()
                        .map(|x| search_values(x, matcher.clone()))
                        .filter(|x| x.is_some())
                        .map(|x| x.unwrap())
                        .flat_map(|x| x)
                        .collect(),
                )
            }
        }
        Value::Object(m) => {
            if matcher.jmatch(json) {
                Some(vec![json])
            } else {
                Some(
                    m.values()
                        .par_bridge()
                        .map(|x| search_values(x, matcher.clone()))
                        .filter(|x| x.is_some())
                        .map(|x| x.unwrap())
                        .flat_map(|x| x)
                        .collect(),
                )
            }
        }
    }
}
