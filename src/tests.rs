#[cfg(test)]
mod tests {
    use crate::opts::Opts;
    use crate::format_json;

    #[test]
    fn test_format_json() {
        // Verify that format works for a simple case.
        let input = String::from("{ \"key\": \"value\", \"list1\": [1, 2, 3, 4], \"list2\": [\"a\", \"b\", \"c\", \"d\"] }");
        let opts = Opts {
            program: String::from("my/tester"),
            program_base: String::from("tester"),
            depth: 32,
            indent: 4,
            input: None,
            output: None,
            verbose: 0,
        };
        let output = format_json(&opts, &input);
        let good = String::from("\
{
    \"key\": \"value\",
    \"list1\": [
        1,
        2,
        3,
        4
    ],
    \"list2\": [
        \"a\",
        \"b\",
        \"c\",
        \"d\"
    ]
}
");
        assert_eq!(output, good);
    }

    #[test]
    fn test_format_json_indent_2() {
        // Verify that format works for a simple case
        // with a non-default indent (2).
        let input = String::from("{ \"key\": \"value\", \"list1\": [1, 2, 3, 4], \"list2\": [\"a\", \"b\", \"c\", \"d\"] }");
        let opts = Opts {
            program: String::from("my/tester"),
            program_base: String::from("tester"),
            depth: 32,
            indent: 2,
            input: None,
            output: None,
            verbose: 0,
        };
        let output = format_json(&opts, &input);
        let good = String::from("\
{
  \"key\": \"value\",
  \"list1\": [
    1,
    2,
    3,
    4
  ],
  \"list2\": [
    \"a\",
    \"b\",
    \"c\",
    \"d\"
  ]
}
");
        assert_eq!(output, good);
    }
}
