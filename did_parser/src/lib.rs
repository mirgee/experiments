extern crate once_cell;
extern crate regex;

mod error;

use error::ParseError;

use std::{collections::HashMap, ops::Range};

type DIDRange = Range<usize>;

#[derive(Default, Debug, Clone)]
pub struct ParsedDID {
    did_url: String,
    did: DIDRange,
    method: DIDRange,
    id: DIDRange,
    path: Option<DIDRange>,
    fragment: Option<DIDRange>,
    queries: HashMap<DIDRange, DIDRange>,
    params: HashMap<DIDRange, DIDRange>,
}

fn parse_key_value(
    did_url: &str,
    start: usize,
    end: usize,
) -> Result<(usize, usize, usize), ParseError> {
    // Skip separator
    let key_start = start + 1;
    // Value starts after equal sign
    // No equal sign is an error
    let value_start = did_url[key_start..end]
        .find('=')
        .map(|i| key_start + i + 1)
        .ok_or(ParseError::InvalidDIDURL)?;
    // Empty key or value is an error
    if value_start == key_start || value_start == end {
        return Err(ParseError::InvalidDIDURL);
    }
    // Value ends at end of string or next separator
    let next_pos = did_url[value_start..end]
        .find(|c: char| c == ';' || c == '?' || c == '#' || c == '/' || c == '&')
        .map_or(end, |i| value_start + i);

    Ok((key_start, value_start, next_pos))
}

fn parse_param(
    did_url: &str,
    current_pos: usize,
) -> Result<(HashMap<DIDRange, DIDRange>, usize), ParseError> {
    let (key_start, value_start, next_pos) = parse_key_value(did_url, current_pos, did_url.len())?;
    let mut params = HashMap::new();
    params.insert(key_start..value_start - 1, value_start..next_pos);
    Ok((params, next_pos))
}

fn parse_query(
    did_url: &str,
    current_pos: usize,
) -> Result<(HashMap<DIDRange, DIDRange>, usize), ParseError> {
    let (key_start, value_start, next_pos) = parse_key_value(did_url, current_pos, did_url.len())?;
    let mut queries = HashMap::new();
    queries.insert(key_start..value_start - 1, value_start..next_pos);
    Ok((queries, next_pos))
}

fn parse_did_method_id(did_url: &str) -> Result<(DIDRange, DIDRange, DIDRange), ParseError> {
    // DID = "did:" method ":" method-specific-id
    let method_start = did_url.find(':').ok_or(ParseError::InvalidDIDURL)?;
    let method_end = did_url[method_start + 1..]
        .find(':')
        .map(|i| i + method_start + 1)
        .ok_or(ParseError::InvalidDIDURL)?;

    // TODO
    // assumed: method-specific-id = 1*idchar
    // actual : method-specific-id = *( *idchar ":" ) 1*idchar
    let id_start = method_end + 1;
    let id_end = did_url[id_start..]
        .find(|c: char| c == ';' || c == '/' || c == '?' || c == '#' || c == '&')
        .map_or(did_url.len(), |i| i + id_start);

    let did = 0..id_end;
    let method = method_start + 1..method_end;
    let id = id_start..id_end;

    // No method-specific-id is an error
    if id.is_empty() {
        return Err(ParseError::InvalidDIDURL);
    }

    Ok((did, method, id))
}

fn parse_path(did_url: &str, current_pos: usize) -> Option<DIDRange> {
    // Path ends with query, fragment, param or end of string
    let path_end = did_url[current_pos..]
        .find(|c: char| c == '?' || c == '#' || c == ';')
        .map_or(did_url.len(), |i| i + current_pos);
    Some(current_pos..path_end)
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
                    let (new_params, next_pos) = parse_param(&did_url, current_pos)?;
                    params.extend(new_params);
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
                    let (new_queries, next_pos) = parse_query(&did_url, current_pos)?;
                    queries.extend(new_queries);
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
            did,
            method,
            id,
            path,
            queries,
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_cases_positive {
        ($($name:ident: $input:expr, $expected_did:expr, $expected_method:expr, $expected_id:expr, $expected_path:expr, $expected_queries:expr, $expected_fragment:expr, $expected_params:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let parsed_did = ParsedDID::parse($input.to_string()).unwrap();

                    assert_eq!(parsed_did.did(), $expected_did, "DID");
                    assert_eq!(parsed_did.method(), $expected_method, "Method");
                    assert_eq!(parsed_did.id(), $expected_id, "ID");
                    assert_eq!(parsed_did.path(), $expected_path, "Path");
                    assert_eq!(parsed_did.queries(), $expected_queries, "Queries");
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
                    println!("Testing {:?}", ParsedDID::parse($input.to_string()));
                    assert!(ParsedDID::parse($input.to_string()).is_err());
                }
            )*
        };
    }

    test_cases_positive! {
        test_case1:
            "did:example:123456789abcdefghi",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            None,
            HashMap::new(),
            None,
            HashMap::new()

        test_case2:
            "did:example:123456789abcdefghi/path",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            Some("/path"),
            HashMap::new(),
            None,
            HashMap::new()

        test_case3:
            "did:example:123456789abcdefghi/path?query1=value1&query2=value2",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            Some("/path"),
            {
                let mut queries = HashMap::new();
                queries.extend(vec![
                    ("query1".to_string(), "value1".to_string()),
                    ("query2".to_string(), "value2".to_string()),
                ]);
                queries
            },
            None,
            HashMap::new()

        test_case4:
            "did:example:123456789abcdefghi/path?query=value#fragment",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            Some("/path"),
            {
                let mut queries = HashMap::new();
                queries.extend(vec![("query".to_string(), "value".to_string())]);
                queries
            },
            Some("fragment"),
            HashMap::new()

        test_case5:
            "did:example:123456789abcdefghi;param1=value1;param2=value2#fragment",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            None,
            HashMap::new(),
            Some("fragment"),
            { let mut params = HashMap::new(); params.extend(vec![("param1".to_string(), "value1".to_string()),("param2".to_string(), "value2".to_string())]); params }

        test_case6:
            "did:example:123456789abcdefghi#fragment",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            None,
            HashMap::new(),
            Some("fragment"),
            HashMap::new()

        test_case7:
            "did:example:123456789abcdefghi?query=value",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            None,
            {
                let mut queries = HashMap::new();
                queries.extend(vec![("query".to_string(), "value".to_string())]);
                queries
            },
            None,
            HashMap::new()

        test_case8:
            "did:example:123456789abcdefghi/path#fragment",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            Some("/path"),
            HashMap::new(),
            Some("fragment"),
            HashMap::new()

        test_case9:
            "did:example:123456789abcdefghi;param=value",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            None,
            HashMap::new(),
            None,
            {
                let mut params = HashMap::new();
                params.extend(vec![("param".to_string(), "value".to_string())]);
                params
            }

        test_case10:
            "did:example:123456789abcdefghi;param=value?query=value",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            None,
            {
                let mut queries = HashMap::new();
                queries.extend(vec![("query".to_string(), "value".to_string())]);
                queries
            },
            None,
            {
                let mut params = HashMap::new();
                params.extend(vec![("param".to_string(), "value".to_string())]);
                params
            }

        test_case11:
            "did:example:123456789abcdefghi/path;param=value",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            Some("/path"),
            HashMap::new(),
            None,
            {
                let mut params = HashMap::new();
                params.extend(vec![("param".to_string(), "value".to_string())]);
                params
            }

        test_case12:
            "did:example:123456789abcdefghi/path?query1=value1;param1=value1&query2=value2#fragment",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            Some("/path"),
            {
                let mut queries = HashMap::new();
                queries.extend(vec![
                    ("query1".to_string(), "value1".to_string()),
                    ("query2".to_string(), "value2".to_string()),
                ]);
                queries
            },
            Some("fragment"),
            {
                let mut params = HashMap::new();
                params.extend(vec![("param1".to_string(), "value1".to_string())]);
                params
            }

        test_case13:
            "did:example:123456789abcdefghi?query=value&query2=#fragment",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            None,
            {
                let mut queries = HashMap::new();
                queries.extend(vec![
                    ("query".to_string(), "value".to_string()),
                    ("query2".to_string(), "".to_string()),
                ]);
                queries
            },
            Some("fragment"),
            HashMap::new()

        test_case14:
            "did:example:123456789abcdefghi;param1=value1;param2=value2?query1=value1&query2=value2#fragment",
            "did:example:123456789abcdefghi",
            "example",
            "123456789abcdefghi",
            None,
            {
                let mut queries = HashMap::new();
                queries.extend(vec![
                    ("query1".to_string(), "value1".to_string()),
                    ("query2".to_string(), "value2".to_string()),
                ]);
                queries
            },
            Some("fragment"),
            {
                let mut params = HashMap::new();
                params.extend(vec![
                    ("param1".to_string(), "value1".to_string()),
                    ("param2".to_string(), "value2".to_string()),
                ]);
                params
            }
    }

    test_cases_negative! {
        test_failure_case1: ""
        test_failure_case2: "not-a-did"
        test_failure_case3: "did:example"
        test_failure_case4: "did:example:123456789abcdefghi;param="
        test_failure_case5: "did:example:123456789abcdefghi?query="
        test_failure_case6: "did:example:123456789abcdefghi/path?query1=value1&query2"
    }
}
