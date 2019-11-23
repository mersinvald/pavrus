## Jorney through Wirth Compiler Construstion in Rust

[![codecov](https://codecov.io/gh/mersinvald/wirth-compiler-construction/branch/master/graph/badge.svg)](https://codecov.io/gh/mersinvald/wirth-compiler-construction)

This repository is my take on basics of compiler construction, while reading Wirth's Compiler Construstion.

### Versioning and branches

`master` branch is latest stable version

`8.1.3-basic-codegen`, where 2 is chapter, 1 is section and 3 is subsection -- represent location in the book and a short summary of an implemented change

This repository follows Semantic Versioning and [Conventional Commits](CONVENTIONAL_COMMITS.md).

Tagging and GitHub releases are automatic, through the use of [semanteecore](https://github.com/semantecore/semantecore) CI tool.

### Artistic differences

* I decided to not call the language Oberon-0, and chose a rather hipsterish name `pavrus`, which translates as `small` from Latin.
* `pavrus` has a Rust-ish syntax, with no begins and ends
* To be continued...