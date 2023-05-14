# intcode-computer

An implementation of the famous [_intcode computer_](https://esolangs.org/wiki/Intcode) from [Advent of Code 2019](https://adventofcode.com/2019).

Set up the `memory`, the `input`, and the `state`. Then run the `program` and extract the `output`.

Example program that echoes one input to output:

```rust
use intcode_computer::{ program::run_program, state::{ Memory, Input, State } };

let memory = Memory::parse("3,0,4,0,99");
let input = Input::parse("123");
let state = State::with_input(memory, input);

let finished_state = run_program(state);

let output = finished_state.output;
assert_eq!(output, vec![123]);
```

The most interesting modules are probably:
- [instruction](src/instruction.rs)
- [program](src/program.rs)

(They also contain most of the unit tests.)

Supporting modules:
- [parameter](src/parameter.rs)
- [state](src/state.rs)

There is also a [CLI](../intcode-computer-cli/) bin crate in the workspace.