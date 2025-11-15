Change Log of convert2json utilities
====================================

Version 2.4.0 / 2025-11-15
--------------------------
- bump bson from 2.15.0 to 3.0.0
  > 3.0 updates several APIs in backwards-incompatible ways; in most cases these
  > changes should require only minor updates in application code.

Version 2.3.8 / 2025-11-15
--------------------------
- bump quick-xml from 0.38.3 to 0.38.4

Version 2.3.7 / 2025-10-18
--------------------------
- bump csv from 1.3.1 to 1.4.0
- bump serde from 1.0.227 to 1.0.228

Version 2.3.6 / 2025-09-27
--------------------------
- bump serde from 1.0.225 to 1.0.227

Version 2.3.5 / 2025-09-20
--------------------------
- bump serde from 1.0.219 to 1.0.225
- bump serde_json from 1.0.143 to 1.0.145
- bump plist from 1.7.4 to 1.8.0

Version 2.3.4 / 2025-08-30
--------------------------
- bump quick-xml from 0.38.2 to 0.38.3

Version 2.3.3 / 2025-08-23
--------------------------
- bump serde_json from 1.0.142 to 1.0.143
- bump quick-xml from 0.38.1 to 0.38.2

Version 2.3.2 / 2025-08-10
--------------------------
- bump serde_json from 1.0.141 to 1.0.142
- bump quick-xml from 0.38.0 to 0.38.1

Version 2.3.1 / 2025-07-19
--------------------------
- bump serde_json from 1.0.140 to 1.0.141
- bump toml from 0.9.0 to 0.9.2
- updated dependencies

Version 2.3.0 / 2025-07-09
--------------------------
- bump plist from 1.7.2 to 1.7.4
- bump toml from 0.8.21 to 0.9.0
  > New TOML parser and writer which carries a risk for regressions
- fix XML trimming, caused by changes in quick-xml 0.38.0

Version 2.2.5 / 2025-07-07
--------------------------
- fix XML entities being dropped, caused by changes in quick-xml 0.38.0

Version 2.2.4 / 2025-07-05
--------------------------
- bump quick-xml from 0.37.5 to 0.38.0

Version 2.2.3 / 2025-06-16
--------------------------
- bump plist from 1.7.1 to 1.7.2

Version 2.2.2 / 2025-05-24
--------------------------
- bump bson from 2.14.0 to 2.15.0
- increased the maximum BSON document size from 16 MiB to the BSON spec maximum
  size of 2^31 - 1 bytes (almost 2 GiB).
- enable and address pedantic clippy lints

Version 2.2.1 / 2025-05-03
--------------------------
- disable unused TOML serialization, reducing compile time & binary size
- in jq/jaq wrappers, if help was requested, skip error reading input message
- switch from loops to iterators for readability
- switch from size to full optimizations in release builds
- bump quick-xml from 0.37.4 to 0.37.5

Version 2.2.0 / 2025-04-27
--------------------------
- JSON maps now preserve order of keys from the input. When using jq/jaq variant
  commands, the keys can be sorted using the `--sort-keys` or `-S` flags.
- added BSON format support
- added CBOR format support
- added MessagePack format support
- added Plist format support
- deduplicate logic
- bumped rust version to 1.81 for bson & half, used in ciborium
- bump toml from 0.8.8 to 0.8.21
- updated dependencies

Version 2.1.0 / 2025-04-11
--------------------------
- fix YAML local tag conversion:
  `key: !Foo bar` becomes `{"key": {"Foo":"bar"}}`
  (instead of `invalid type: enum` error)
- bump quick-xml from 0.37.2 to 0.37.4

Version 2.0.0 / 2025-03-31
--------------------------
- blending quick-xml with xmltojson library for XML deserialization
  This is a major, breaking change and the resulting JSON will have a different
  structure. In particular:
  - empty elements get a value of `null` instead of an empty object:
    `<root/>` becomes `{"root":null}` (instead of `{"root": {}}`)
  - simple text nodes are now values of their keys: `<key>value</key>` becomes
    `{"key":"value"}` (instead of `{"key":{"$text":"value"}}`)
  - sequences of tags and text nodes get preserved instead of overwriting
    each other (#91):
    `<tag>A <inner>B</inner><inner>C</inner> D <inner>E</inner></tag>` becomes
    `{"tag":["A",{"inner":["B","C"]},"D",{"inner":"E"}]}`
    (instead of `{"tag":{"$text":"D","inner":{"$text":"E"}}}`)

Version 1.1.6 / 2025-03-15
--------------------------
- bump serde from 1.0.218 to 1.0.219
- bump serde_json from 1.0.137 to 1.0.140

Version 1.1.5 / 2025-01-25
--------------------------
- bump serde_json from 1.0.135 to 1.0.137

Version 1.1.4 / 2025-01-11
--------------------------
- bump quick-xml from 0.37.1 to 0.37.2
- bump serde_json from 1.0.134 to 1.0.135

Version 1.1.3 / 2024-12-28
--------------------------
- bump serde from 1.0.215 to 1.0.217
- bump serde_json from 1.0.133 to 1.0.134
- updated dependencies

Version 1.1.2 / 2024-11-23
--------------------------
- bump serde_json from 1.0.132 to 1.0.133
- bump quick-xml from 0.37.0 to 0.37.1
- updated dependencies

Version 1.1.1 / 2024-11-16
--------------------------
- bump csv from 1.3.0 to 1.3.1
- bump serde from 1.0.214 to 1.0.215

Version 1.1.0 / 2024-11-02
--------------------------
- bump serde from 1.0.213 to 1.0.214
- bump quick-xml from 0.36.2 to 0.37.0, which introduces the following change:
  > Handle only those boolean representations that are allowed by Xml Schema
  > which is only `"true"`, `"1"`, `"false"`, and `"0"`. Previously the
  > following values also was accepted:
  > | `bool`  | XML content                                                 |
  > | ------- | ----------------------------------------------------------- |
  > | `true`  | `"True"`, `"TRUE"`, `"t"`, `"Yes"`, `"YES"`, `"yes"`, `"y"` |
  > | `false` | `"False"`, `"FALSE"`, `"f"`, `"No"`, `"NO"`, `"no"`, `"n"`  |

Version 1.0.7 / 2024-10-26
--------------------------
- bump serde from 1.0.210 to 1.0.213
- bump serde_json from 1.0.131 to 1.0.132

Version 1.0.6 / 2024-10-19
--------------------------
- bump serde_json from 1.0.128 to 1.0.131

Version 1.0.5 / 2024-09-21
--------------------------
- bump quick-xml from 0.36.1 to 0.36.2

Version 1.0.4 / 2024-09-08
--------------------------
- bump serde from 1.0.209 to 1.0.210
- bump serde_json from 1.0.127 to 1.0.128

Version 1.0.3 / 2024-08-31
--------------------------
- bump serde from 1.0.208 to 1.0.209
- bump serde_json from 1.0.125 to 1.0.127

Version 1.0.2 / 2024-08-17
--------------------------
- bump serde from 1.0.205 to 1.0.208
- bump serde_json from 1.0.122 to 1.0.125

Version 1.0.1 / 2024-08-10
--------------------------
- bump serde from 1.0.204 to 1.0.205

Version 1.0.0 / 2024-08-03
--------------------------
- switching to maintained quick-xml library for XML deserialization
  This is a major, breaking change and the resulting JSON will have a different
  structure. In particular:
  - document root node gets preserved as the outermost objects only key (#48):
    `<root/>` becomes `{"root": {}}` (instead of `{}`)
  - attributes start with an @-character to distinguish them from inner tags:
    `<tag attribute="value"><inner/></tag>` becomes
    `{"tag":{"@attribute":"value","inner":{}}}`
    (instead of `{"attribute":"value","inner":{}}`)
  - text nodes are now called `$text` instead of `$value`:
    `<key>value</key>` becomes `{"key":{"$text":"value"}}`
    (instead of `{"$value":"value"}`)

Version 0.9.3 / 2024-08-03
--------------------------
- bump serde_json from 1.0.120 to 1.0.122

Version 0.9.2 / 2024-07-16
--------------------------
- bump serde from 1.0.203 to 1.0.204

Version 0.9.1 / 2024-07-06
--------------------------
- bump serde_json from 1.0.117 to 1.0.120

Version 0.9.0 / 2024-06-17
--------------------------
- added RSV format support
- bump serde from 1.0.201 to 1.0.203

Version 0.8.5 / 2023-05-14
--------------------------
- bump serde from 1.0.200 to 1.0.201
- bump serde_json from 1.0.116 to 1.0.117

Version 0.8.4 / 2023-05-04
--------------------------
- bump serde from 1.0.198 to 1.0.200

Version 0.8.3 / 2023-04-21
--------------------------
- bump serde from 1.0.197 to 1.0.198
- bump serde_json from 1.0.115 to 1.0.116

Version 0.8.2 / 2023-03-30
--------------------------
- bump serde_yaml from 0.9.33 to 0.9.34
  Note that this library has now been declared deprecated by it's maintainer and
  we'll be looking for a carefully chosen replacement - do raise an issue if you
  have any suggestions, needs to serde deserialize yaml
- bump serde_json from 1.0.114 to 1.0.115

Version 0.8.1 / 2023-03-23
--------------------------
- bumped rust version to 1.74 for clap 4.5.1, used in rpm packaging

Version 0.8.0 / 2023-03-23
--------------------------
- added support to wrap jaq in addition to jq, jaq is preferred with a fallback
  to jq if not found
- bump serde_yaml from 0.9.32 to 0.9.33

Version 0.7.2 / 2023-02-25
--------------------------
- bump serde from 1.0.196 to 1.0.197
- bump serde_json from 1.0.113 to 1.0.114
- bump serde_yaml from 0.9.31 to 0.9.32

Version 0.7.1 / 2023-02-03
--------------------------
- bump serde from 1.0.195 to 1.0.196
- bump serde_json from 1.0.112 to 1.0.113
- bump serde_yaml from 0.9.30 to 0.9.31

Version 0.7.0 / 2023-01-25
--------------------------
- added INI format support
- on `cq --help`, display both csv2json & jq usage

Version 0.6.2 / 2023-01-24
--------------------------
- minor cleanup, re-add -? & -help argument support to csv2json, lost in 0.4.0
  due to using argh lib
- use array instead of HashMap, trade in jq arg parsing speed for smaller binary

Version 0.6.1 / 2023-01-23
--------------------------
- switching to simpler, but for this use case more flexible, pico-args library
- simplified & deduplicated argument parsing logic

Version 0.6.0 / 2023-01-21
--------------------------
- added csv2json argument to disable string trimming: starting and trailing
  whitespace characters will get removed from strings by default, ex. "foo, bar"
  will now yield "bar", not " bar", can be disabled with --no-trim
- added support to cq for the same CSV related arguments as csv2json
- added support for integer and floating point numbers when parsing CSV files
- skip filenames passed to jq arguments, ex. --from-file

Version 0.5.6 / 2023-01-13
--------------------------
- bump serde from 1.0.192 to 1.0.195
- bump serde_json from 1.0.108 to 1.0.111
- bump serde_yaml from 0.9.29 to 0.9.30

Version 0.5.5 / 2023-12-28
--------------------------
- fix feature selection
- switch to the std-library is_terminal function
- increase minimum rust version to 1.70
- bump toml from 0.8.2 to 0.8.8

Version 0.5.4 / 2023-12-23
--------------------------
- bump serde_yaml from 0.9.27 to 0.9.29

Version 0.5.3 / 2023-11-18
--------------------------
- bump serde from 1.0.190 to 1.0.192
- bump serde_json from 1.0.107 to 1.0.108
- bump serde_yaml from 0.9.25 to 0.9.27

Version 0.5.2 / 2023-10-28
--------------------------
- bump csv from 1.2.2 to 1.3.0
- bump rustix from 0.38.9 to 0.38.19
  ([GHSA-c827-hfw6-qwvm](https://github.com/advisories/GHSA-c827-hfw6-qwvm))
- bump toml from 0.8.1 to 0.8.2
- bump serde from 1.0.188 to 1.0.190

Version 0.5.1 / 2023-10-01
--------------------------
- bump toml from 0.8.0 to 0.8.1

Version 0.5.0 / 2023-09-18
--------------------------
- bump toml from 0.7.7 to 0.8.0, changes tuple variants from being an array to
  being a table with the key being the variant name and the value being the
  array, matching serde_json's behavior
- bump serde_json from 1.0.105 to 1.0.107

Version 0.4.8 / 2023-09-10
--------------------------
- bump toml from 0.7.6 to 0.7.7

Version 0.4.7 / 2023-08-28
--------------------------
- bump serde from 1.0.183 to 1.0.188
- bump serde_json from 1.0.104 to 1.0.105

Version 0.4.6 / 2023-08-12
--------------------------
- bump argh from 0.1.10 to 0.1.12
- bump serde from 1.0.180 to 1.0.183

Version 0.4.5 / 2023-08-01
--------------------------
- bump serde from 1.0.179 to 1.0.180

Version 0.4.4 / 2023-07-31
--------------------------
- bump serde from 1.0.175 to 1.0.179
- bump serde_json from 1.0.103 to 1.0.104

Version 0.4.3 / 2023-07-24
--------------------------
- bump serde from 1.0.171 to 1.0.175
- bump serde_json from 1.0.100 to 1.0.103
- bump serde_yaml from 0.9.22 to 0.9.25, fixes serializing using quoted style
  around scalar that has digits with leading zero

Version 0.4.2 / 2023-07-10
--------------------------
- bump serde from 1.0.164 to 1.0.171, which bumps syn from 2.0.18 to 2.0.25
- bump serde_json from 1.0.99 to 1.0.100
- bump toml from 0.7.5 to 0.7.6
- bump is-terminal from 0.4.8 to 0.4.9

Version 0.4.1 / 2023-07-03
--------------------------
- bump is-terminal from 0.4.7 to 0.4.8, which bumps rustix from 0.37 to 0.38,
  changing some APIs and improving compile-time

Version 0.4.0 / 2023-07-02
--------------------------
- added csv2json arguments for changing default delimiter, quote & escape
  characters

Version 0.3.0 / 2023-07-02
--------------------------
- added support for files in arguments to *2json tools
- added YAML multi-document support

Version 0.2.0 / 2023-07-01
--------------------------
- added CSV format support

Version 0.1.0 / 2023-06-25
--------------------------
- initial release, covering TOML, XML and YAML formats and for each a tool for
  converting from STDIN to STDOUT and a jq wrapper
