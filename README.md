# BrainPipe 
BrainPipe is a Brainfuck variant that is meant to separate the fun part
of writing brainfuck programs, which is writing novel turing-machine-esque
solutions for a task, from the boring and frustrating part, which is 
large-scale memory management and input handling. 

It currently adds one feature: pipes.
Pipes can be placed between two or more functionally independent 
(with closed brackets) brainfuck programs, whose inputs and outputs
are then chained together. The first is connected to stdin and the last
is connected to stdout. The last program is run until it requires input,
at which point the program above it will run until it has provided this input,
or has requested input itself. If any of these brainfuck programs ends, the entire thing will. 

## Building the Webassembly target
Doing this requires you to [have wasm-pack installed](https://rustwasm.github.io/wasm-pack/installer/). You can then run 
```
./build-wasm.sh /path/to/web-project/
``` 
(on linux) to have the artifact written into your web project directly, and then interact with the WebAssembly according to the `--target web` [way of doing things.](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm#compiling_our_code_to_webassembly)
This creates two symbolic links, one called "pkg" which is used as the target to write the .wasm and .js files to, 
and "js" which goes to the root of the web project and can be used to get function-exporting files accessible by wasm-pack and/or wasm-bingen with an attribute such as this:
```rust
#[wasm_bindgen(module = "/js/wasm_imports.js")]
extern {
    //...
}
```

