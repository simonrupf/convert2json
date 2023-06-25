convert2json
============

Utilities to convert TOML, XML & YAML to JSON for use on the command line. For
each supported format there is a tool for use as a pipe as well as a jq wrapper
which forwards the converted piped input or detected files in the arguments to
jq, for further querying and processing.

Goals:
+ provide light-weight converters to JSON
+ provide jq wrappers
+ add support for additional formats having maintained Serde implementations

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
- [ ] support files in arguments to *2json tools
- [ ] yaml multi-document support
- [ ] provide RPM packages

Feature Matrix
--------------
You may not want to install all of the utilities or only a particular one. Each
utility can be selected by itself or via group features. If no features are
selected, all utilities get installed.

Matrix of all selectable features:
|      | to_json   | jq |
|------|-----------|----|
| toml | toml2json | tq |
| xml  | xml2json  | xq |
| yaml | yaml2json | yq |

Examples:
 ```
 # install only yq & tq:
 cargo install convert2json --no-default-features --features yq,tq
 # install all xml tools:
 cargo install convert2json --no-default-features --features xml
 ```
