//! Unit tests.

#[cfg(test)]
mod tests {
    use crate::opts::Opts;
    use crate::format;

    /// Verify that format works for a simple case.
    #[test]
    fn test_format_json() {
        let input = String::from("{ \"key\": \"value\", \"list1\": [1, 2, 3, 4], \"list2\": [\"a\", \"b\", \"c\", \"d\"] }");
        let program = String::from("/test/format/json/tester");
        let opts = Opts::defaults(&program);
        let output = format(&opts, &input);
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
        assert_eq!(opts.program_base, "tester".to_string());
    }

    /// Verify that format works for a simple case
    /// with a non-default indent (2).
    #[test]
    fn test_format_json_indent_2() {
        let input = String::from("{ \"key\": \"value\", \"list1\": [1, 2, 3, 4], \"list2\": [\"a\", \"b\", \"c\", \"d\"] }");
        let program = String::from("/test/format/json/tester");
        let mut opts = Opts::defaults(&program);
        opts.indent = 2;
        let output = format(&opts, &input);
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
