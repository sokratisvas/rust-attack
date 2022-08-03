# Virtual Machine to Hack Assembly Translator
A VM translator for the Hack computer. It supports stack arithmetic and boolean operations, if/goto statements, function declarations, calls and return statements.
# Quick Start 
You can translate a single .vm file to assembly:
```sh
cargo run vm-translator examples/PointerTest.vm
```
Or you can translate a directory with multiple .vm files:
```
FibonacciElement/
		Main.vm
		Sys.vm
```
```sh
cargo run vm-translator examples/FibonacciElement
```
