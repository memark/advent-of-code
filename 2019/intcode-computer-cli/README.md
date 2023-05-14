# intcode-computer-cli

An accompanying CLI for the [intcode-computer](../intcode-computer/) lib crate. The CLI is bulit on `clap` and the binary is called `icc`.

Call the CLI specifying the memory and the inputs. The outputs will be printed to `stdout`.

```sh
$ icc "3,0,4,0,3,0,4,0,99" "123,234"
123,234
```

For some syntax help run
```sh
$ icc -h
Usage: icc <MEMORY> <INPUT>

Arguments:
  <MEMORY>  memory (instructions and data) as a comma-separated list of ints, e.g. "3,0,4,0,99"
  <INPUT>   input as a comma-separated list of ints, e.g. "1,2,3"

Options:
  -h, --help  Print help
```