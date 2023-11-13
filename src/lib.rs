mod api;
mod client;
mod collection;
mod index;
mod document;
mod database;
mod params;
mod responses;
mod query;
mod model;

use std::{time::SystemTime, any::type_name};
use convert_case::{Case, Casing};

pub use client::*;
pub use index::*;
use regex::Regex;
pub use responses::*;
pub use database::*;
pub use params::*;
pub use query::*;
pub use model::*;
pub use document::*;
pub use collection::*;


pub fn get_current_timestamp() -> u64 {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as u64;
}

pub  fn get_default_expiration() -> u64 {
    return get_current_timestamp() + (86400 * 30 * 6); // 6 months
}

fn extract_name(name: String) -> String {
    // error: look-around, including look-ahead and look-behind, is not supported
    // let pattern = r#"(?<=<).+(?=>)"#;

    // only extract nested type if outer is Doc or Edge
    if name.contains("Doc") || name.contains("Edge") {
        let pattern: &str = r#"<.+>"#;
        let regex: Regex = Regex::new(pattern).unwrap();
    
        if let Some(captured) = regex.find(&name) {
            // Access the captured text using .as_str()
            let result = captured.as_str();
            let sanitized: &str = &result[1..result.len() - 1];
            return extract_name(sanitized.to_owned());
        }
    }

    return name.split("<").next().unwrap().split("::").last().to_owned().unwrap().to_owned().to_case(Case::Snake);
    // return name.split("::").last().to_owned().unwrap().to_owned().to_case(Case::Snake);
}

/// primary schema naming convention
/// Convert SomeType to snake case some_type
/// 
/// TODO: Fix for R = Edge<T> -> should use T name instead of 'edge'
pub fn name<R>() -> String {
    let full_name: String = type_name::<R>().to_owned();
    return extract_name(full_name);
}

pub fn collection_name<R>() -> String {
    return name::<R>();
}

pub trait Arango {
    fn new(config: &Config) -> Self;
}

impl Arango for Database {
    fn new(config: &Config) -> Self {
        Self {
            client: Client::new(config)
        }
    }
}

impl Arango for Document {
    fn new(config: &Config) -> Self {
        Self {
            client: Client::new(config)
        }
    }
}

impl Arango for Query {
    fn new(config: &Config) -> Self {
        Self {
            client: Client::new(config)
        }
    }
}

impl Arango for Index {
    fn new(config: &Config) -> Self {
        Self {
            client: Client::new(config)
        }
    }
}