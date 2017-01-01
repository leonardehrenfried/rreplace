// open.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::fs::metadata;

extern crate glob;
extern crate gitignore;
use glob::glob;

fn main() {
    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    let pwd = env::current_dir().unwrap();
    let gitignore_path = pwd.join(".gitignore");
    let gitignore_file = gitignore::File::new(&gitignore_path).unwrap();

    for path in glob("**/*").unwrap()
        .filter_map(Result::ok)
        .map(|p| pwd.join(&p))
        .filter(|p| metadata(&p).unwrap().is_file())
        .filter(|p| !gitignore_file.is_excluded(&p).unwrap()) {
        println!("{}", path.display());

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                                                       why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display,
                                                       why.description()),
            Ok(_) => print!("{} contains:\n{}", display, s.replace("bar", "quox")),
        }
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
