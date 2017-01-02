# search_replace
Recursively search and replace strings in text files

This program searches through a tree of (text) files and replaces all occurances of a string with another one. Think of 
it as a combination of `grep` and `sed`.

```
$ search_replace --help
search_replace 0.1.0
Leonard Ehrenfried <leonard.ehrenfried@gmail.com>

USAGE:
    search_replace <to_replace> <replace_with>

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