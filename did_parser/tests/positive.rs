use did_parser::ParsedDID;
use std::collections::HashMap;

macro_rules! test_cases_positive {
    ($($name:ident: $input:expr, $expected_did:expr, $expected_method:expr, $expected_id:expr, $expected_path:expr, $expected_fragment:expr, $expected_queries:expr, $expected_params:expr)*) => {
        $(
            #[test]
            fn $name() {
                let parsed_did = ParsedDID::parse($input.to_string()).unwrap();

                assert_eq!(parsed_did.did(), $expected_did, "DID");
                assert_eq!(parsed_did.method(), $expected_method, "Method");
                assert_eq!(parsed_did.id(), $expected_id, "ID");
                assert_eq!(parsed_did.path(), $expected_path, "Path");
                assert_eq!(parsed_did.fragment(), $expected_fragment, "Fragment");
                assert_eq!(parsed_did.queries(), $expected_queries, "Queries");
                assert_eq!(parsed_did.params(), $expected_params, "Params");
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
        None,
        HashMap::new(),
        HashMap::new()

    test_case2:
        "did:example:123456789abcdefghi/path",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        Some("/path"),
        None,
        HashMap::new(),
        HashMap::new()

    test_case3:
        "did:example:123456789abcdefghi/path?query1=value1&query2=value2",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        Some("/path"),
        None,
        {
            let mut queries = HashMap::new();
            queries.extend(vec![
                ("query1".to_string(), "value1".to_string()),
                ("query2".to_string(), "value2".to_string()),
            ]);
            queries
        },
        HashMap::new()

    test_case4:
        "did:example:123456789abcdefghi/path?query=value#fragment",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        Some("/path"),
        Some("fragment"),
        {
            let mut queries = HashMap::new();
            queries.extend(vec![("query".to_string(), "value".to_string())]);
            queries
        },
        HashMap::new()

    test_case5:
        "did:example:123456789abcdefghi;param1=value1;param2=value2#fragment",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        None,
        Some("fragment"),
        HashMap::new(),
        {
            let mut params = HashMap::new();
            params.extend(vec![
                ("param1".to_string(), "value1".to_string()),
                ("param2".to_string(), "value2".to_string())
            ]);
            params
        }

    test_case6:
        "did:example:123456789abcdefghi#fragment",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        None,
        Some("fragment"),
        HashMap::new(),
        HashMap::new()

    test_case7:
        "did:example:123456789abcdefghi?query=value",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        None,
        None,
        {
            let mut queries = HashMap::new();
            queries.insert("query".to_string(), "value".to_string());
            queries
        },
        HashMap::new()

    test_case8:
        "did:example:123456789abcdefghi/path#fragment",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        Some("/path"),
        Some("fragment"),
        HashMap::new(),
        HashMap::new()

    test_case9:
        "did:example:123456789abcdefghi;param=value",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        None,
        None,
        HashMap::new(),
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
        None,
        {
            let mut queries = HashMap::new();
            queries.insert("query".to_string(), "value".to_string());
            queries
        },
        {
            let mut params = HashMap::new();
            params.insert("param".to_string(), "value".to_string());
            params
        }

    test_case11:
        "did:example:123456789abcdefghi/path;param=value",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        Some("/path"),
        None,
        HashMap::new(),
        {
            let mut params = HashMap::new();
            params.insert("param".to_string(), "value".to_string());
            params
        }

    test_case12:
        "did:example:123456789abcdefghi/path?query1=value1;param1=value1&query2=value2#fragment",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        Some("/path"),
        Some("fragment"),
        {
            let mut queries = HashMap::new();
            queries.extend(vec![
                ("query1".to_string(), "value1".to_string()),
                ("query2".to_string(), "value2".to_string()),
            ]);
            queries
        },
        {
            let mut params = HashMap::new();
            params.insert("param1".to_string(), "value1".to_string());
            params
        }

    test_case13:
        "did:example:123456789abcdefghi?query=value&query2=#fragment",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        None,
        Some("fragment"),
        {
            let mut queries = HashMap::new();
            queries.extend(vec![
                ("query".to_string(), "value".to_string()),
                ("query2".to_string(), "".to_string()),
            ]);
            queries
        },
        HashMap::new()

    test_case14:
        "did:example:123456789abcdefghi;param1=value1;param2=value2?query1=value1&query2=value2#fragment",
        "did:example:123456789abcdefghi",
        "example",
        "123456789abcdefghi",
        None,
        Some("fragment"),
        {
            let mut queries = HashMap::new();
            queries.extend(vec![
                ("query1".to_string(), "value1".to_string()),
                ("query2".to_string(), "value2".to_string()),
            ]);
            queries
        },
        {
            let mut params = HashMap::new();
            params.extend(vec![
                ("param1".to_string(), "value1".to_string()),
                ("param2".to_string(), "value2".to_string()),
            ]);
            params
        }
}