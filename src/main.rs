#![deny(warnings)]
// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::fs::metadata;

extern crate gitignore;
extern crate clap;

use clap::{App, Arg, ArgMatches};

fn main() {
    let pwd = env::current_dir().unwrap();
    let gitignore_path = pwd.join(".gitignore");
    let gitignore_file = gitignore::File::new(&gitignore_path).unwrap();

    let params = parse_arguments();

    for path in gitignore_file.included_files().unwrap().iter()
        .map(|p| pwd.join(&p))
        .filter(|p| metadata(&p).unwrap().is_file()) {
        println!("{}", path.display());

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => {
                println!("skipped {}: {}", path.display(), why.description())
            }
            Ok(_) => {
                // Open and read the file entirely
                drop(file);  // Close the file early

                // Run replace operation in memory
                let new_data = s.replace(&*params.to_replace, &*params.replace_with);

                // Recreate the file and dump the processed contents to it
                let mut f = File::create(&path).expect("Unable to create file");
                f.write_all(new_data.as_bytes()).expect("Unable to write data");
            }
        }
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

struct Params {
    to_replace: String,
    replace_with: String,
}

fn unwrap_arg(matches: &ArgMatches, name: &str) -> String {
    matches.args.get(name).unwrap().vals[1].clone().into_string().unwrap()
}

fn parse_arguments() -> Params {
    let x = App::new("search_replace")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Leonard Ehrenfried <leonard.ehrenfried@gmail.com>")

        // We'll add two positional arguments, a input file, and a config file.
        //
        // I'll explain each possible setting that "positionals" accept. Keep in
        // mind that you DO NOT need to set each of these for every flag, only the
        // ones that apply to your individual case.
        .arg(Arg::with_name("to_replace")
                    .help("the string to replace") // Displayed when showing help info
                    .index(1)                      // Set the order in which the user must
                    .required(true)                // By default this argument MUST be present
        )
        .arg(Arg::with_name("replace_with")
            .help("the string to replace it with")
            .index(2)
            .required(true)
        )                     // Note, we do not need to specify required(true)
        // if we don't want to, because "input" already
        // requires "config"
        // Note, we also do not need to specify requires("input")
        // because requires lists are automatically two-way

        // NOTE: In order to compile this example, comment out mutually_excludes()
        // because we have not defined an "output" argument.
        .get_matches();

    let to_replace = unwrap_arg(&x, "to_replace");
    let replace_with = unwrap_arg(&x, "replace_with");

    Params { to_replace: to_replace, replace_with: replace_with }

}