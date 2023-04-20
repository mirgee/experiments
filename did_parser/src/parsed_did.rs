use std::{collections::HashMap, str::FromStr};

use crate::{
    error::ParseError,
    utils::parse::{parse_did_method_id, parse_key_value, parse_path},
    DIDRange,
};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ParsedDID {
    did_url: String,
    did: Option<DIDRange>,
    method: Option<DIDRange>,
    id: Option<DIDRange>,
    path: Option<DIDRange>,
    fragment: Option<DIDRange>,
    queries: HashMap<DIDRange, DIDRange>,
    params: HashMap<DIDRange, DIDRange>,
}

impl ParsedDID {
    pub fn parse(did_url: String) -> Result<Self, ParseError> {
        let (did, method, id) = parse_did_method_id(&did_url)?;

        let mut path = None;
        let mut fragment = None;
        let mut queries = HashMap::new();
        let mut params = HashMap::new();

        let mut current_pos = id.end;

        while current_pos < did_url.len() {
            match did_url.chars().nth(current_pos) {
                Some(';') => {
                    let (key_start, value_start, next_pos) =
                        parse_key_value(&did_url, current_pos, did_url.len())?;
                    params.insert(key_start..value_start - 1, value_start..next_pos);
                    current_pos = next_pos;
                }
                Some('/') => {
                    if path.is_none() {
                        path = parse_path(&did_url, current_pos);
                        current_pos = path.as_ref().unwrap().end;
                    } else {
                        current_pos += 1;
                    }
                }
                Some('?') | Some('&') => {
                    let (key_start, value_start, next_pos) =
                        parse_key_value(&did_url, current_pos, did_url.len())?;
                    queries.insert(key_start..value_start - 1, value_start..next_pos);
                    current_pos = next_pos;
                }
                Some('#') => {
                    if fragment.is_none() {
                        fragment = Some(current_pos + 1..did_url.len());
                    }
                    current_pos += 1;
                }
                _ => break,
            };
        }

        Ok(ParsedDID {
            did_url,
            did: Some(did),
            method: Some(method),
            id: Some(id),
            path,
            queries,
            fragment,
            params,
        })
    }

    pub fn did(&self) -> Option<&str> {
        self.did.clone().map(|range| self.did_url[range].as_ref())
    }

    pub fn method(&self) -> Option<&str> {
        self.method
            .clone()
            .map(|range| self.did_url[range].as_ref())
    }

    pub fn id(&self) -> Option<&str> {
        self.id.clone().map(|range| self.did_url[range].as_ref())
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_ref().map(|path| &self.did_url[path.clone()])
    }

    pub fn queries(&self) -> HashMap<String, String> {
        self.queries
            .iter()
            .map(|(k, v)| {
                (
                    self.did_url[k.clone()].to_string(),
                    self.did_url[v.clone()].to_string(),
                )
            })
            .collect()
    }

    pub fn fragment(&self) -> Option<&str> {
        self.fragment
            .as_ref()
            .map(|fragment| &self.did_url[fragment.clone()])
    }

    pub fn params(&self) -> HashMap<String, String> {
        self.params
            .iter()
            .map(|(k, v)| {
                (
                    self.did_url[k.clone()].to_string(),
                    self.did_url[v.clone()].to_string(),
                )
            })
            .collect()
    }
}

impl FromStr for ParsedDID {
    type Err = ParseError;

    fn from_str(did_url: &str) -> Result<Self, Self::Err> {
        Self::parse(did_url.to_string())
    }
}
