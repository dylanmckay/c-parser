# c-parser
##A parser for the C language.

It supports:
* Preprocessor
  * `#define ident [expression]`
  * `#define ident(a,b,c) [expression]`
* Comments (for now, the actual comment strings are ignored).

It is very incomplete.

It is technically a library, but while I am writing it, it is set up to compile to a binary.
