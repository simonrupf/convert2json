Change Log of convert2json utilities
====================================

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
