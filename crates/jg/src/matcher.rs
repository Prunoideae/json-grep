use std::vec;

use rayon::prelude::*;
use regex::Regex;
use serde_json::{Map, Value};

pub fn compile_regex(jr: &Value) -> Result<Box<dyn JsonMatcher + Send + Sync>, String> {
    match jr {
        Value::String(s) => {
            Ok(Box::new(StringMatcher::new(s)?) as Box<dyn JsonMatcher + Send + Sync>)
        }
        Value::Array(v) => {
            Ok(Box::new(ArrayMatcher::new(v)?) as Box<dyn JsonMatcher + Send + Sync>)
        }
        Value::Object(m) => {
            Ok(Box::new(ObjectMatcher::new(m)?) as Box<dyn JsonMatcher + Send + Sync>)
        }
        _ => Err(format!(
            "Error parsing value {:?}, unsupported type of value.",
            jr
        )),
    }
}

pub trait JsonMatcher: Send + Sync {
    fn jmatch(&self, entry: &Value) -> bool;
}

struct StringMatcher {
    regex: Regex,
}

impl StringMatcher {
    pub fn new(regex: &String) -> Result<Self, String> {
        Ok(StringMatcher {
            regex: match Regex::new(regex.as_str()) {
                Ok(n) => n,
                Err(_) => {
                    return Err(format!("Error when compiling \"{}\" as regex.", regex));
                }
            },
        })
    }
}

impl JsonMatcher for StringMatcher {
    fn jmatch(&self, entry: &Value) -> bool {
        match entry {
            Value::Null => false,
            Value::Bool(v) => self.regex.as_str() == v.to_string(),
            Value::Number(v) => self.regex.is_match(v.to_string().as_str()),
            Value::String(v) => self.regex.is_match(v.as_str()),
            Value::Array(_) => false,
            Value::Object(_) => false,
        }
    }
}

pub struct ArrayMatcher {
    submatchers: Vec<Box<dyn JsonMatcher + Send + Sync>>,
}

impl ArrayMatcher {
    pub fn new(json_regex: &Vec<Value>) -> Result<Self, String> {
        let mut submatchers = vec![];
        for i in json_regex {
            submatchers.push(compile_regex(i)?);
        }
        Ok(ArrayMatcher { submatchers })
    }
}

impl JsonMatcher for ArrayMatcher {
    fn jmatch(&self, entry: &Value) -> bool {
        match entry {
            Value::Array(v) => self
                .submatchers
                .par_iter()
                .all(|x| v.iter().any(|y| x.jmatch(y))),
            _ => false,
        }
    }
}
pub struct ObjectMatcher {
    submatchers: Vec<(Regex, Box<dyn JsonMatcher + Send + Sync>)>,
}

impl ObjectMatcher {
    pub fn new(json_regex: &Map<String, Value>) -> Result<Self, String> {
        let mut submatchers = vec![];
        for (k, v) in json_regex {
            submatchers.push((
                match Regex::new(k) {
                    Ok(n) => n,
                    Err(_) => {
                        return Err(format!("Error when parsing \"{}\" as regex.", k));
                    }
                },
                compile_regex(v)?,
            ))
        }
        Ok(ObjectMatcher { submatchers })
    }
}

impl JsonMatcher for ObjectMatcher {
    fn jmatch(&self, entry: &Value) -> bool {
        match entry {
            Value::Object(m) => self
                .submatchers
                .par_iter()
                .all(|(r, sm)| m.iter().any(|(k, v)| r.is_match(k) && sm.jmatch(v))),
            _ => false,
        }
    }
}
