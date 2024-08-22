# time-rs
Simple CLI program for timing how long it takes a command to run `x` amount of times

## Usage
`timers <command>` - *Runs `<command>` once*

By default, this only runs the command once. The `-t,--times` parameter allows you to specify how many times you would like for it to be ran:

`timers -t 1000 <command>` - *Runs `<command>` 1000 times*

Additionally, `<command>` does not write to `stdout` by default. To enable `<command>` to write to `stdout`, you can use the `-o,--stdout` flag:

`timers -t 1000 -o <command>` - *Runs `<command>` 1000 times, and also writes its output to `stdout`*
