//! # json-fmt
//! This tool formats JSON to make it more readable or simpler to
//! analyze using tools like grep.
//!
//! # Program Help
//! Help that describes how to use the program can be found by
//! specifying the `--help` option.
//!
//! # Program Documentation
//! How to generate this documentation.
//!<pre>
//!   $ cargo doc --no-deps --document-private-items  --color always
//!</pre>
//! To read it, first start a server of some sort.
//!<pre>
//!   $ python -m http.server -d target/doc 8080
//!</pre>
//! Then navigate to the following URL: http://localhost:8080/json_fmt/
use std::process;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{self, Read, Write};

mod util;

mod opts;
use opts::{Opts};

mod tests;

/// The primary entry point called by `main()` in `main.rs`.
///
/// It reads the input into a string, formats into another string and
/// then writes it out. All of the data is stored in memory which will
/// be a challenge for very large data files.
///
/// # Arguments
/// * `cli` - A string vector that contains the command line arguments.
///
/// # Example
/// <pre>
/// use std::env;
/// use json_fmt::{run};
/// fn main() {
///    let args: Vec<String> = env::args().collect();
///    run(&args);
/// }
/// </pre>
pub fn run(cli: &Vec<String>) {
    let opts = Opts::new(cli);
    let input = read(&opts);
    let output = format(&opts, &input);
    write(&opts, &output);
    infov!(opts, 1, "done");
}

/// Read the input.
///
/// It accepts input from stdin or from a file and returns the data
/// read. It determines whether stdin should be read by looking for
/// the `--input` argument. If it is present, then the input is
/// assumed to be a file, otherwise it is stdin.
///
/// # Arguments
/// * `opts` - The command line arguments.
///
/// # Returns
/// The input data as a string.
pub fn read(opts: &Opts) -> String {
    let mut data = String::new();
    if let Some(x) = &opts.input {
        // File.
        infov!(opts, 1, "reading from file {}", x);
        let path = Path::new(&x);
        data = match fs::read_to_string(path) {
            Ok(x) => x,
            Err(e) => {
                err!("{}", e);
            },
        };
    } else {
        // Stdin.
        infov!(opts, 1, "reading from stdin");
        match io::stdin().read_to_string(&mut data) {
            Ok(_) => {},
            Err(e) => {
                err!("read from stdin failed: {}", e);
            },
        };
    }
    infov!(opts, 1, "read {} bytes", data.len());
    data
}

/// Format the JSON.
///
/// It reads the input string and formats it.
///
/// The parser is very tolerant of failure. It does not care if
/// the input is valid JSON.
///
/// There is a limit to the nesting depth that can be controlled by
/// the `--depth` options.
///
/// # Arguments
/// * `opts` - The command line arguments.
/// * `input` - The input data.
///
/// # Returns
/// * The formatted JSON string.
pub fn format(opts: &Opts, input: &String) -> String {
    // Setup the indents.
    let mut indents = Vec::<String>::new();
    let mut indent_prefix = String::new();
    for _ in 0..opts.depth {
        indents.push(indent_prefix.to_string());
        for _ in 0..opts.indent {
            indent_prefix.push(' ');
        }
    }

    // Format the JSON.
    let mut output = String::new();
    let vc: Vec<char> = input.chars().collect();
    let size = vc.len() as usize;
    let mut max_nest = 0;
    let mut indent = 0 as i32;
    let mut nl = false;  // flag to indicate a newline is needed
    let mut i = 0 as usize;
    while i < size {
        if vc[i] == '{' || vc[i] == '[' {
            if nl {
                output.push('\n');
                output += &indents[indent as usize];
            }
            output.push(vc[i]);
            indent += 1;
            nl = true;
            if indent as usize >= opts.depth {
                err!("maximum depth ({}) exceeded, increase the --depth option and try again", opts.depth);
            }
            if max_nest < indent {
                max_nest = indent;
            }
        } else if vc[i] == '}' || vc[i] == ']' {
            indent -= 1;
            assert!(indent >= 0);
            output.push('\n');
            output += &indents[indent as usize];
            output.push(vc[i]);
        } else if vc[i] == ',' {
            output.push(vc[i]);
            nl = true;
        } else if vc[i] == ':' {
            output.push(vc[i]);
            output.push(' ');
            nl = false;
        } else if vc[i] == '"' {
            // Traverse to the end of the string.
            if nl == true {
                output.push('\n');
                output += &indents[indent as usize];
                nl = false;
            }
            output.push(vc[i]);
            i += 1;
            while i < size && vc[i] != '"' {
                if vc[i] == '\\' {
                    output.push(vc[i]);
                    i += 1
                }
                output.push(vc[i]);
                i += 1;
            }
            output.push(vc[i]);  // get the last quote
        } else if vc[i] == ' ' || vc[i] == '\t' || vc[i] == '\n' {
            // Skip white space, that is what the formatting
            // is controlling.
        } else {
            if nl == true {
                output.push('\n');
                output += &indents[indent as usize];
                nl = false;
            }
            output.push(vc[i]);
        }
        i += 1
    }
    let last_char = output.chars().last().unwrap();
    if last_char != '\n' {
        output.push('\n');
    }
    infov!(opts, 1, "max-nesting-level: {}", max_nest);
    output
}

/// Write the formatted JSON out.
///
/// It writes the formatted JSON to either stdout or a file depending
/// on whether the `--output` option is set. If it is set, then it is
/// assumed to be a file, otherwise it is stdout.
///
/// # Arguments
/// * `opts` - The command line arguments.
/// * `input` - The input data.
///
pub fn write(opts: &Opts, output: &String) {
    if let Some(x) = &opts.output {
        // File.
        infov!(opts, 1, "writing {} bytes to file {}", output.len(), x);
        let mut fp = File::create(&x).unwrap();
        write!(&mut fp, "{}", output).unwrap();
    } else {
        // Stdout.
        infov!(opts, 1, "writing {} bytes to stdout", output.len());
        print!("{}", output);
    }
}
