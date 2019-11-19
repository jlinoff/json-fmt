# json-fmt
Simple, fast JSON file formatter written in Rust

This tool formats JSON to make it more readable or simpler to analyze using tools like grep.

Here is a very simple example:
```bash
$ cat test.json
{ "key": "value", "list1": [1, 2, 3, 4], "list2": ["a", "b", "c", "d"] }
$ cat test.json | json-fmt
{
    "key": "value",
    "list1": [
        1,
        2,
        3,
        4
    ],
    "list2": [
        "a",
        "b",
        "c",
        "d"
    ]
}
```

For more information about how to use the tool specify the help (`-h`) option.

A common use case is to look for fields in a large number of unformatted JSON files.

Here is an example of analyzing a color field in 10000 unformatted JSON files.
```bash
$ time find files -type f | \
    head -10000 | \
    xargs -L1 -I{} ~/bin/json-fmt -i {} | \
    grep '"colors"' | \
    awk -F'"' '{print $4}' | \
    sort -f | \
    uniq -c
   4 magenta
 996 red
 844 blue
 152 green
```

I wrote it as fast replacement for the tools i have been using and as an exercise in learning Rust.
