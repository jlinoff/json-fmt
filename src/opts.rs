//! Command line options handling and help.
use std::env;
use std::process;
use std::path::Path;
use std::collections::VecDeque;

// From the Cargo.toml file.
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
//const LICENSE: &'static str = env!("CARGO_PKG_LICENSE"); // not available!
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

// This macro is used by the command line processing
// logic to check for the existence of an argument
// for an option that requires it. An example would
// be "-m 10".
#[allow(unused_macros)]
macro_rules! check_arg {
    ($o:expr, $i:expr, $m:expr) => (
        $i += 1;
        if $i >= $m {
            err!("missing argument for {}", $o);
        }
    );
}

pub struct Opts {
    pub program: String,
    pub program_base: String,
    pub depth: usize,
    pub indent: usize,
    pub input: Option<String>,
    pub output: Option<String>,
    pub verbose: usize,
}

impl Opts {
    pub fn new(cli: &Vec<String>) -> Opts {
        let mut opts = Opts::defaults(&cli[0]);
        let base = opts.program_base.to_string();

        // The pending vector allows us to handle things
        // like -vv, and -vvI 3.
        let mut pending: VecDeque<String> = VecDeque::new();
        let mut i = 1;
        let mut opt;
        while i < cli.len() || pending.len() > 0 {
            if pending.len() > 0 {
                opt = pending.pop_front().unwrap();
                i -= 1;  // make sure that arg processing works
            } else {
                opt = cli[i].to_string();
            }

            //info!("debug: i={}, opt={} cli[{}]={} len={}", i, &opt, i, cli[i], cli.len());
            if opt == "-h" || opt == "--help" {
                Opts::help(&base)
            } else if opt == "-i" || opt == "--input" {
                check_arg!(opt, i, cli.len());
                let arg = &cli[i];
                if let Some(_) = opts.input {
                    err!("the option --input can only be specified once");
                }
                opts.input = Some(arg.to_string());
            } else if opt == "-I" || opt == "--indent" {
                check_arg!(opt, i, cli.len());
                let arg = &cli[i];
                opts.indent = match arg.parse::<usize>() {
                    Ok(m) => m,
                    Err(e) => {err!("invalid indent: '{}' - {}", arg, e);},
                };
            }
            else if opt == "-n" || opt == "--nesting-level" {
                check_arg!(opt, i, cli.len());
                let arg = &cli[i];
                opts.depth = match arg.parse::<usize>() {
                    Ok(m) => m,
                    Err(e) => {err!("invalid depth: '{}' - {}", arg, e);},
                };
            } else if opt == "-o" || opt == "--output" {
                check_arg!(opt, i, cli.len());
                let arg = &cli[i];
                if let Some(_) = opts.output {
                    err!("the option --output can only be specified once");
                }
                opts.output = Some(arg.to_string());
            } else if opt == "-v" || opt == "--verbose" {
                opts.verbose += 1;
            } else if opt == "-V" || opt == "--version" {
                println!("{} {}", base, VERSION);
                process::exit(0);
            } else if opt.len() > 2 && opt.starts_with("-") {
                // Allow -vvh, etc.
                // This is cheating from a pedantic perspective
                // because UTF-8 is assumed. May change it later.
                let chars: Vec<char> = opt.chars().collect();
                for j in 1..chars.len() {
                    let newopt = format!("-{}", chars[j]);
                    pending.push_back(newopt);
                }
                info!("pending.len() == {}", pending.len());
            } else {
                err!("unrecognized option: '{}'", opt);
            }
            i += 1;
        }
        opts
    }

    pub fn defaults(program: &String) -> Opts {
        let base_ostr = Path::new(&program).file_stem().unwrap();
        let base = base_ostr.to_str().unwrap().to_string();
        let opts = Opts {
            program: program.to_string(),
            program_base: base.to_string(),
            depth: 32,
            indent: 4,
            input: None,  // stdin
            output: None,  // stdout
            verbose: 0,
        };
        opts
    }

    fn help(prog: &String) {
        println!("\
\x1b[1mUSAGE\x1b[0m
    {p} [OPTIONS]

\x1b[1mDESCRIPTION\x1b[0m
    This tool formats JSON to make it more readable or simpler to
    analyze using tools like grep.

    A common use case is to look for fields in a large number of
    unformatted JSON files. Here is an example of analyzing a
    color field in 10000 unformatted JSON files.

        $ find files -type f | \\
            head -10000 | \\
            xargs -L1 -I{{}} ~/bin/json-fmt -i {{}} | \\
            grep '\"colors\"' | \\
            awk -F'\"' '{{print $4}}' | \\
            sort -f | \\
            uniq -c
           4 magenta
         996 red
         844 blue
         152 green

\x1b[mOPTIONS\x1b[0m
    -h, --help         Print this help message and exit.

    -i, --input FILE   The input file name. If this is not specified
                       the  input JSON data is read from stdin.

    -I, --indent NUM   The preferred indentation. The default is 4.

    -n, --nesting-level NUM
                       The maximum nesting level (depth).
                       If the maximum depth is exceeded, the program
                       will fail.
                       The default is 32.

    -o, --output FILE  The output file name. If this is not specified
                       the formatted JSON data is written to stdout.

    -v, --verbose      Increase the level of verbosity.
                       You almost always want at least `-v` because
                       that prints summary information.

    -V, --version      Print the program version and exit.

\x1b[1mEXAMPLES\x1b[0m
    # Example 1: get help
    $ {p} -h

    # Example 2: read from stdin, write to stdout
    $ cat test.json
    {{ \"key\": \"value\", \"list1\": [1, 2, 3, 4], \"list2\": [\"a\", \"b\", \"c\", \"d\"] }}
    $ cat test.json | {p}
    {{
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
    }}

    # Example 3: read from file, write to stdout
    $ {p} -i test.json
    .
    .

    # Example 4: read from file, write to file
    $ {p} -i test.json -o out.json

    # Example 5: change the indent level to 2
    $ {p} -I 2 -i test.json -o out.json

    # Example 6: see what the program would do
    #            using the verbose option
    $ {p} -v -i test.json -o /dev/null
    INFO:src/lib.rs:25: reading from file test.json
    INFO:src/lib.rs:43: read 2380 bytes
    INFO:src/lib.rs:129: max-nesting-level: 4
    INFO:src/lib.rs:137: writing 3263 bytes to file /dev/null
    INFO:src/lib.rs:17: done

\x1b[1mVERSION\x1b[0m
    {v}

\x1b[1mLICENSE\x1b[0m
    MIT Open Source

\x1b[1mAUTHORS\x1b[0m
    {a}
", p=prog, v=VERSION, a=AUTHORS);
        process::exit(0);
    }
}
