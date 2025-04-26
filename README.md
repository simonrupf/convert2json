convert2json
============
Utilities for use on the command line, to convert CBOR, CSV, INI, MessagePack,
RSV, TOML, XML & YAML to JSON. For each supported format there is a tool for use
in a pipe as well as a wrapper which passes the converted input or files in the
arguments to jaq or jq, for further querying and processing.

Overview
--------
Goals:
+ provide light-weight converters to JSON
+ provide jaq and jq wrappers
+ add support for additional formats having maintained [Serde](https://serde.rs/) implementations

Non-Goals:
- converting JSON into other data formats, consider [jyt](https://github.com/ken-matsui/jyt)
- replicating [jq](https://jqlang.github.io/jq/), jaq or jq must be installed to
use the jq wrappers

Usage examples
--------------
```
# convert yaml to json
$ echo foo: bar | yaml2json
{"foo":"bar"}

# query a value from a toml file
$ tq -r .package.description Cargo.toml
CLI utilities to convert CBOR, CSV, INI, MessagePack, RSV, TOML, XML & YAML into JSON and for use with jaq or jq.

# query for environment variables that contain the current users username, using ini parser
$ printenv | iq --compact-output '.USER as $user | with_entries(select(.value | contains($user))) | keys'
["HOME","LOGNAME","OLDPWD","PWD","USER","USERNAME"]

# csv2json & cq recognize 4 additional arguments
$ csv2json --help
Usage: csv2json [-d <delimiter>] [-q <quote>] [-E <escape>] [--no-trim] [files...]

Reads CSV from files or standard input and converts this to JSON, emitted on standard output. Any errors are reported to standard error and result in a non-zero exit code.

Options:
  -d, --delimiter   field delimiter to use when parsing CSV, defaults to: ,
                    (comma)
  -q, --quote       quote character to use when parsing CSV, defaults to: "
                    (double quote)
  -E, --escape      escape character to use when parsing CSV, to escape quote
                    characters within a field. By default, quotes get escaped by
                    doubling them.
  --no-trim         do not trim headers & fields. By default, both get trimmed
                    of starting or trailing whitespace characters.
  -h, --help        display usage information
```

Alternatives
------------
* Rust 🦀:
  * [jyt](https://github.com/ken-matsui/jyt)
  * [yaml2json](https://github.com/dafu-wu/yaml2json)
  * [yaml2json-rs](https://github.com/Nessex/yaml2json-rs)
* [Go 🐹](https://pkg.go.dev/search?q=yaml2json&m=)
* [JavaScript 🌐](https://www.npmjs.com/search?q=yaml2json)
* [Python 🐍](https://pypi.org/search/?q=yaml2json)

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
|             | to_json    | jq     |
|-------------|----------- |--------|
| bson        | bson2json  | bq     |
| cbor        | cbor2json  | cborq  |
| csv         | csv2json   | cq     |
| messagepack | msg2json   | msgq   |
| plist       | plist2json | plistq |
| ini         | ini2json   | iq     |
| rsv         | rsv2json   | rq     |
| toml        | toml2json  | tq     |
| xml         | xml2json   | xq     |
| yaml        | yaml2json  | yq     |

Examples:
 ```
 # install only yq & tq:
 cargo install convert2json --no-default-features --features yq,tq
 # install all the xml related tools:
 cargo install convert2json --no-default-features --features xml
 ```
