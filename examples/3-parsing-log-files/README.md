# Error Log Analyzer

An example Rust project to read from a log file and report error count per hour. It has several deficiencies like hard coding the format for the timestamps and the keyword to determine if a line should be considered an error or not.

This is an example line in a log file that would match:

```text
2023-08-30 07:06:11-04 baccatum softwareupdated[298]: Create & persist stash finished. Error: (null)
```
