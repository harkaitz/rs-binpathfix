BINPATHFIX
==========

Small utility in rust for replacing paths in binary files.

## Help

    Usage: binpathfix [-v][-o OLDPATH][-n NEWPATH] [FILES...]

    Replace OLDPATH (by default current directory) with "/tmp/HASH" in
    the specified binary files (normally ELF executables).

    The hash is computed from the OLDPATH, If -n is specified a symlink
    is created from "/tmp/HASH" to the NEWPATH.

    If -v is specified, the program will print the changes it makes.

## Collaborating

For making bug reports, feature requests and donations visit
one of the following links:

1. [gemini://harkadev.com/oss/](gemini://harkadev.com/oss/)
2. [https://harkadev.com/oss/](https://harkadev.com/oss/)
