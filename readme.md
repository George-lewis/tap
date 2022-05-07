# Tap

An alternative for the standard linux tool `tee`

The syntax is simple: `tap [filename]`, it will copy stdin to `filename` and to stdout

For example: `cat file | tap tapped.txt` will output the contents of `file` to `tapped.txt` and stdout