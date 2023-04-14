use did_parser::ParsedDID;

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

test_cases_negative! {
    test_failure_case1: ""
    test_failure_case2: "not-a-did"
    test_failure_case3: "did:example"
    test_failure_case4: "did:example:123456789abcdefghi;param="
    test_failure_case5: "did:example:123456789abcdefghi?query="
    test_failure_case6: "did:example:123456789abcdefghi/path?query1=value1&query2"
}
