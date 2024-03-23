Change Log of convert2json utilities
====================================

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
