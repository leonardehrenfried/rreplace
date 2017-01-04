# rreplace
Recursively search and replace strings in text files

[![Build Status](https://travis-ci.org/leonardehrenfried/rreplace.svg?branch=master)](https://travis-ci.org/leonardehrenfried/rreplace)

This program searches through a tree of (text) files and replaces all occurances of a string with another one. Think of 
it as a combination of `grep` and `sed`.

```
$ rreplace --help
rreplace 0.1.0
Leonard Ehrenfried <leonard.ehrenfried@gmail.com>

USAGE:
    rreplace <to_replace> <replace_with>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <to_replace>      the string to replace
    <replace_with>    the string to replace it with
```

### Why not use `grep` and `sed`?

I did. Really, I tried. 

For years I was using the above combination but ran into way too many edge cases mostly related to quoting regex characters
that I grew frustrated and started to write my own.
