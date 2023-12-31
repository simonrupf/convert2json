convert2json
============

Utilities for use on the command line, to convert CSV, TOML, XML & YAML to JSON.
For each supported format there is a tool for use in a pipe as well as a jq
wrapper which passes the converted input or files in the
arguments to jq, for further querying and processing.

Usage examples:
```
$ echo foo: bar | yaml2json
{"foo":"bar"}
$ tq -r .package.description Cargo.toml
CLI utilities to convert CSV, TOML, XML & YAML into JSON on standard output or into jq.
```

Overview
--------

Goals:
+ provide light-weight converters to JSON
+ provide jq wrappers
+ add support for additional formats having maintained [Serde](https://serde.rs/) implementations

Non-Goals:
- converting JSON into other data formats, consider [jyt](https://github.com/ken-matsui/jyt)
- replicating [jq](https://jqlang.github.io/jq/), jq must be installed to use the jq wrappers

Alternatives:
* Rust 🦀:
  * [jyt](https://github.com/ken-matsui/jyt)
  * [yaml2json](https://github.com/dafu-wu/yaml2json)
  * [yaml2json-rs](https://github.com/Nessex/yaml2json-rs)
* [Go 🐹](https://pkg.go.dev/search?q=yaml2json&m=)
* [JavaScript 🌐](https://www.npmjs.com/search?q=yaml2json)
* [Python 🐍](https://pypi.org/search/?q=yaml2json)

To Do:
- [ ] in jq arguments, ignore filenames if preceeded by certain flags (i.e. --from-file)

Installation
------------
Packages are provided (statically linked) for Debian & Ubuntu, as wells as RPM
based Linux distributions and for
[Alpine Linux](https://pkgs.alpinelinux.org/packages?name=convert2json).

For Ubuntu, MacOS (universal binaries for x86_64 & arm64) and Windows, archives
with pre-built (dynamically linked) binaries are provided, for manual installation.

You can find these packages and archives in the
[releases](https://github.com/simonrupf/convert2json/releases).

If you have Rust and Cargo installed, you can build and update these tools using
`cargo install convert2json`. See the feature matrix below, if you only need a
subset of the utilities.

Feature Matrix
--------------
You may not want to install all of the utilities or only a particular one. Each
utility can be selected by itself or via group features. If no features are
selected, all utilities get installed.

Matrix of all selectable features:
|      | to_json   | jq |
|------|-----------|----|
| csv  | csv2json  | cq |
| toml | toml2json | tq |
| xml  | xml2json  | xq |
| yaml | yaml2json | yq |

Examples:
 ```
 # install only yq & tq:
 cargo install convert2json --no-default-features --features yq,tq
 # install all the xml related tools:
 cargo install convert2json --no-default-features --features xml
 ```
