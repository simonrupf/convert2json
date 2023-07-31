Change Log of convert2json utilities
====================================

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
- added csv2json arguments for changing default delimiter, quote & escape characters

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
