# odesli

This is a CLI tool built using [odesli-rs](https://crates.io/crates/odesli-rs) to interact with Odesli API

- Tool Help
```
Usage: odesli [OPTIONS] <COMMAND>

Commands:
  get-url    find matches using a URL
  get-id     find matches using entity IDs
  platforms
  help       Print this message or the help of the given subcommand(s)

Options:
  -k, --api-key <api-key>  The Odesli API key to use, if any
  -j, --json               Dump the output in JSON as received from API
  -h, --help               Print help
  -V, --version            Print version
```

- Sample Runs
  1. By URL
```sh
$ odesli --api-key <OPTIONAL_API_KEY_HERE> get-url "https://music.youtube.com/watch?v=cnnOwLfAxn0"
```
  2. By ID
```sh
$ odesli --api-key <OPTIONAL_API_KEY_HERE> get-id 7CNUefGBVLn4cLoYv3ej8x spotify song
```
  3. List available platforms
```sh
$ odesli platforms
```
