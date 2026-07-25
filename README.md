# ASCII Conversion Tool

## Use Case:
- Offline conversion between ASCII text, Decimal, Octal, Hexadecimal, and Binary.

## Get the Tool
- The pre-compiled binary (for Linux and Windows) and source code are available in "Releases".
- macOS users will need to compile from source.

## Running the CLI
`$ ./ascii_converter`

- Follow the prompts to select which format to convert from and to.
- Type or paste in the message you wish to convert.
- ***WARNING: Take great care when pasting in text from untrusted sources, as it may be an attempt to run malicious commands on your device.***
- Ensure the program has executable permissions.

## Building from Source
Navigate to the project root directory.
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

The executable binary should then be available in `./target/release/`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# ASCII Conversion Tool
export PATH="$PATH:/home/path/to/directory/where/this/program/lives"

alias ac="ascii_converter"
```

## License
