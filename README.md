# WASMonkey

The WASMonkey patches a WASM object file to replace a set of exported
functions with imported functions from another library.

## Usage

```text
    wasmonkey [OPTIONS] --builtins <builtins_file> --input <input_file> --output <output_file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --builtins <builtins_file>            Path to the builtins library
    -m, --builtins-map <builtins_map_file>    Path to the builtins map file
    -i, --input <input_file>                  Path to the input file
    -o, --output <output_file>                Path to the output file
```

`builtins_file` is an object containing alternative implementations to
funcitons called in the wasm code.

Symbols starting with a `builtin_` prefix will be used for substitution.

For example, a `memmove()` function defined as an external in the WASM
object will be replaced by calls to an imported `builtin_memmove()`
function, if `builtin_memmove()` is present in the builtins file.

A JSON-encoded map of the performed substitutions can be optionally written
into `builtins_map_file`.
