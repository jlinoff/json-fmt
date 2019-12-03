[![Releases](https://img.shields.io/github/release/jlinoff/json-fmt.svg?style=flat)](https://github.com/jlinoff/json-fmt/releases)

# json-fmt
Simple, fast JSON file formatter written in Rust

This tool formats JSON to make it more readable or simpler to analyze using tools like grep.

For information about how to use the tool specify the help (`-h`) option.

> I wrote it as fast replacement for the tools i have been using and as
> an exercise for learning Rust. I hope you find it useful.

## Download and Build
Here is how you could download and build the tool locally if you do not already have
Rust installed.

```bash
$ # Install Rust locally.
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
$ source ~/.cargo/env

$ # Clone out this repo.
$ git clone https://github.com/jlinoff/json-fmt.git

$ # Build the tool.
$ cd json-fmt
$ cargo build --release
$ cargo test --release

$ # Copy the tool to a local bin:
$ sudo cp target/release/json-fmt /usr/local/bin/json-fmt
$ sudo chmod 0755 /usr/local/bin/json-fmt
$ json-fmt --version
json-fmt 0.2.0

$ # Get help.
json-fmt --help
```

## Simple example
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

## Simple example with an indent of 2
The default indentation is 4 but sometimes it is useful to have a
smaller indent. The example below shows an indent of 2 (specified by
the -I option).
```bash
$ cat test.json
{ "key": "value", "list1": [1, 2, 3, 4], "list2": ["a", "b", "c", "d"] }
$ cat test.json | json-fmt -I 2
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

## Example that uses -v to get information
This example shows how to get information about the JSON file.
```bash
$ cat test.json
{ "key": "value", "list1": [1, 2, 3, 4], "list2": ["a", "b", "c", "d"] }
$ json-fmt -I 2 -i test.json -o /dev/null -v
INFO:src/lib.rs:25: reading from file test.json
INFO:src/lib.rs:43: read 73 bytes
INFO:src/lib.rs:129: max-nesting-level: 2
INFO:src/lib.rs:137: writing 119 bytes to file /dev/null
INFO:src/lib.rs:17: done
```
This can be useful for seeing what the formatting will do without actually
changing anything.

## A more complex example
A common use case is to look for fields in a large number of unformatted JSON files.

Here is an example of analyzing a color field in 10000 unformatted JSON files.
```bash
$ find files -type f | \
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
