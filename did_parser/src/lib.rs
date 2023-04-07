extern crate once_cell;
extern crate regex;

mod error;

use error::ParseError;

use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

// TODO:: We don't want to allocate unnecessarily, but self-referencing
// or referencing the original did is not great either. Ouroboros introduces
// too much complexity, so we might store the original did as a string
// and the other fields could be tuples of indeces into the original string.
pub struct ParsedDID<'a> {
    pub did: &'a str,
    pub did_url: &'a str,
    pub method: &'a str,
    pub id: &'a str,
    pub path: Option<&'a str>,
    pub fragment: Option<&'a str>,
    pub query: Option<&'a str>,
    pub params: HashMap<&'a str, &'a str>,
}

pub struct DIDParser;

static DID_MATCHER: Lazy<Regex> = Lazy::new(|| {
    let pct_encoded = r"(?:%[0-9a-fA-F]{2})";
    let id_char = &format!(r"(?:[a-zA-Z0-9._-]|{})", pct_encoded);
    let param_char = r"[a-zA-Z0-9_.:%-]";
    let param = &format!(r";{}+=({})*", param_char, param_char);

    let method = r"([a-z0-9]+)";
    let method_id = &format!(r"((?:{}:)*({}+))", id_char, id_char);
    let params = &format!(r"(({})*)", param);
    let path = r"(/[^#?]*)?";
    let query = r"([\?]([^#]*)?)?";
    let fragment = r"([#](.*)?)?";

    let did_regex_str = &format!(r"^did:{}:{}{}{}{}{}$", method, method_id, params, path, query, fragment);

    Regex::new(did_regex_str).unwrap()

});

impl<'a> ParsedDID<'a> {
    pub fn parse(did_url: &'a str) -> Result<Self, ParseError> {
       if did_url.is_empty() {
            return Err(ParseError::InvalidDIDURL);
        }

        let sections = DID_MATCHER.captures(&did_url).ok_or(ParseError::InvalidDIDURL)?;

        let method = sections.get(1).map(|m| m.as_str()).unwrap();
        let id = sections.get(2).map(|m| m.as_str()).unwrap();
        let did = &did_url[..sections.get(2).unwrap().end()];

        let params = {
            let mut params = HashMap::new();
            if let Some(param_str) = sections.get(5).map(|m| m.as_str()) {
                for param in param_str.split(';')
                    .filter(|p| !p.is_empty()) {
                    let kv: Vec<&str> = param.splitn(2, '=').collect();
                    if kv.len() == 2 {
                        params.insert(kv[0], kv[1]);
                    } else {
                        return Err(ParseError::InvalidDIDURL);
                    }
                }
            }
            params
        };

        let parsed_did = Self {
            did,
            did_url,
            method,
            id,
            path: sections.get(7).map(|m| m.as_str()),
            query: sections.get(9).map(|m| m.as_str()),
            fragment: sections.get(11).map(|m| m.as_str()),
            params,
        };

        Ok(parsed_did)
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
                    let parsed_did = ParsedDID::parse($input).unwrap();

                    assert_eq!(parsed_did.did, $expected_did, "DID");
                    assert_eq!(parsed_did.method, $expected_method, "Method");
                    assert_eq!(parsed_did.id, $expected_id, "ID");
                    assert_eq!(parsed_did.path, $expected_path, "Path");
                    assert_eq!(parsed_did.query, $expected_query, "Query");
                    assert_eq!(parsed_did.fragment, $expected_fragment, "Fragment");
                    assert_eq!(parsed_did.params, $expected_params, "Params");
                }
            )*
        };
    }

    macro_rules! test_cases_negative {
        ($($name:ident: $input:expr)*) => {
            $(
                #[test]
                fn $name() {
                    assert!(ParsedDID::parse($input).is_err());
                }
            )*
        };
    }


    test_cases_positive! {
        test_case1: "did:example:123456789abcdefghi", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", None, None, None, HashMap::new()
        test_case2: "did:example:123456789abcdefghi/path", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", Some("/path"), None, None, HashMap::new()
        test_case3: "did:example:123456789abcdefghi/path?query=value", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", Some("/path"), Some("query=value"), None, HashMap::new()
        test_case4: "did:example:123456789abcdefghi/path?query=value#fragment", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", Some("/path"), Some("query=value"), Some("fragment"), HashMap::new()
        test_case5: "did:example:123456789abcdefghi;param=value", "did:example:123456789abcdefghi", "example", "123456789abcdefghi", None, None, None, { let mut params = HashMap::new(); params.insert("param", "value"); params }
    }

    test_cases_negative! {
        test_failure_case1: ""
        test_failure_case2: "not-a-did"
        test_failure_case3: "did:example"
    }
}
