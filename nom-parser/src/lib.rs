#[cfg(test)]
mod tests {
    use nom::bytes::complete::{tag, take_while1};
    use nom::character::complete::{alpha1, alphanumeric1, digit1};
    use nom::character::{is_alphabetic, is_alphanumeric};
    use nom::combinator::opt;
    use nom::multi::{separated_list0, separated_list1};
    use nom::sequence::pair;
    use nom::IResult;

    #[test]
    fn test_tag() {
        fn match_hello(input: &str) -> IResult<&str, &str> {
            tag("hello")(input)
        }
        let input = "hello, world!";
        let result = match_hello(input);
        println!("{:?}", result);
    }

    #[test]
    fn test_parse_digits() {
        fn parse_digits(input: &str) -> IResult<&str, &str> {
            digit1(input)
        }
        let input = "1a1a1a1a1";
        let result = parse_digits(input);
        println!("{:?}", result);
    }

    #[test]
    fn test_parse_list() {
        fn parse(input: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag(", "), alphanumeric1)(input)
        }
        let input = "hello, world!";
        let result = parse(input);
        println!("{:?}", result);
    }

    #[test]
    fn test_parse_pair() {
        fn parse(input: &str) -> IResult<&str, (&str, &str)> {
            pair(alpha1, digit1)(input)
        }
        let input = "aaa111aaa";
        let result = parse(input);
        println!("{:?}", result);
    }

    #[test]
    fn test_parse_optional() {
        fn parse(input: &str) -> IResult<&str, Option<&str>> {
            opt(tag("!"))(input)
        }
        let input = "!hey";
        let result = parse(input);
        println!("Opt1: {:?}", result);
        let input = "hey";
        let result = parse(input);
        println!("Opt2: {:?}", result);
    }

    #[test]
    fn test_parse_take_while() {
        fn is_alphanumeric(c: char) -> bool {
            c.is_alphanumeric()
        }
        fn parse1(input: &str) -> IResult<&str, &str> {
            take_while1(is_alphanumeric)(input)
        }
        fn parse2(input: &[u8]) -> IResult<&[u8], &[u8]> {
            take_while1(is_alphabetic)(input)
        }
        let input = "aaa111a";
        let result = parse1(input);
        println!("{:?}", result);
    }
}
