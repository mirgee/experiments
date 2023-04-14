extern crate once_cell;
extern crate regex;

mod error;

use error::ParseError;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, ops::Range};

type DIDRange = Range<usize>;

#[derive(Default)]
pub struct ParsedDID {
    did_url: String,
    did: Range<usize>,
    method: Range<usize>,
    id: Range<usize>,
    path: Option<Range<usize>>,
    query: Option<Range<usize>>,
    fragment: Option<Range<usize>>,
    params: HashMap<Range<usize>, Range<usize>>,
}

fn parse_param(rest: &str, start: usize, end: usize) -> Result<(usize, usize, usize), ParseError> {
    let key_start = start + 1;
    let value_start = rest[key_start..end]
        .find('=')
        .map_or(end, |i| key_start + i + 1);
    if value_start != end && value_start == key_start {
        return Err(ParseError::InvalidDIDURL);
    }
    let next_pos = rest[value_start..end]
        .find(|c: char| c == ';' || c == '?' || c == '#' || c == '/')
        .map_or(end, |i| value_start + i);

    Ok((key_start, value_start, next_pos))
}

impl ParsedDID {
    pub fn parse(did_url: String) -> Result<Self, ParseError> {
        let method_start = did_url.find(':').ok_or(ParseError::InvalidDIDURL)?;
        let mut iter = did_url[method_start + 1..].char_indices();
        let method_end = iter
            .by_ref()
            .find(|&(_, c)| c == ':')
            .map(|(i, _)| i + method_start + 1)
            .ok_or(ParseError::InvalidDIDURL)?;

        let id_start = method_end + 1;
        let id_end = iter
            .by_ref()
            .find(|&(_, c)| c == ';' || c == '/' || c == '?' || c == '#')
            .map_or(did_url.len(), |(i, _)| i + method_start + 1);

        let did = 0..id_end;
        let method = method_start + 1..method_end;
        let id = id_start..id_end;

        let mut path = None;
        let mut query = None;
        let mut fragment = None;
        let mut params = HashMap::new();

        let mut current_pos = id_end;

        while current_pos < did_url.len() {
            match did_url.chars().nth(current_pos) {
                Some(';') => {
                    let (key_start, value_start, next_pos) =
                        parse_param(&did_url, current_pos, did_url.len())?;
                    params.insert(key_start..value_start - 1, value_start..next_pos);
                    current_pos = next_pos;
                }
                Some('/') => {
                    if path.is_none() {
                        let path_end = did_url[current_pos..]
                            .find(|c: char| c == '?' || c == '#' || c == ';')
                            .map_or(did_url.len(), |i| i + current_pos);
                        path = Some(current_pos..path_end);
                        current_pos = path_end;
                    } else {
                        current_pos += 1;
                    }
                }
                Some('?') => {
                    if query.is_none() {
                        let query_end = did_url[current_pos..]
                            .find(|c: char| c == '#' || c == ';')
                            .map_or(did_url.len(), |i| i + current_pos);
                        query = Some(current_pos + 1..query_end);
                        current_pos = query_end;
                    } else {
                        current_pos += 1;
                    }
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
            did,
            method,
            id,
            path,
            query,
            fragment,
            params,
        })
    }

    pub fn did(&self) -> &str {
        &self.did_url[self.did.clone()]
    }

    pub fn method(&self) -> &str {
        &self.did_url[self.method.clone()]
    }

    pub fn id(&self) -> &str {
        &self.did_url[self.id.clone()]
    }

    pub fn path(&self) -> Option<&str> {
        self.path.as_ref().map(|path| &self.did_url[path.clone()])
    }

    pub fn query(&self) -> Option<&str> {
        self.query
            .as_ref()
            .map(|query| &self.did_url[query.clone()])
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_cases_positive {
        ($($name:ident: $input:expr, $expected_did:expr, $expected_method:expr, $expected_id:expr, $expected_path:expr, $expected_query:expr, $expected_fragment:expr, $expected_params:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let parsed_did = ParsedDID::parse($input.to_string()).unwrap();

                    assert_eq!(parsed_did.did(), $expected_did, "DID");
                    assert_eq!(parsed_did.method(), $expected_method, "Method");
                    assert_eq!(parsed_did.id(), $expected_id, "ID");
                    assert_eq!(parsed_did.path(), $expected_path, "Path");
                    assert_eq!(parsed_did.query(), $expected_query, "Query");
                    assert_eq!(parsed_did.fragment(), $expected_fragment, "Fragment");
                    assert_eq!(parsed_did.params(), $expected_params, "Params");
                }
            )*
        };
    }

    macro_rules! test_cases_negative {
        ($($name:ident: $input:expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert!(ParsedDID::parse($input.to_string()).is_err());
                }
            )*
        };
    }

    test_cases_positive! {
        test_case1: "did:example:123456789abcdefghi", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", None, None, None, HashMap::new()
        test_case2: "did:example:123456789abcdefghi/path", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", Some("/path"), None, None, HashMap::new()
        test_case3: "did:example:123456789abcdefghi/path?query=value", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", Some("/path"), Some("query=value"), None, HashMap::new()
        test_case4: "did:example:123456789abcdefghi/path?query=value#fragment", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", Some("/path"), Some("query=value"), Some("fragment"), HashMap::new()
        test_case5: "did:example:123456789abcdefghi;param1=value1;param2=value2", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", None, None, None, { let mut params = HashMap::new(); params.extend(vec![("param1".to_string(), "value1".to_string()),("param2".to_string(), "value2".to_string())]); params }
    }

    test_cases_negative! {
        test_failure_case1: ""
        test_failure_case2: "not-a-did"
        test_failure_case3: "did:example"
    }
}
