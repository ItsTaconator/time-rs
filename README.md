# time-rs
Simple CLI program for timing how long it takes a command to run `x` amount of times

![timers screenshot](https://taconator.com/static/img/projects/timers.avif)
*<p align="center">Screenshot generated with <a href="https://github.com/homeport/termshot">termshot</a></p>*

## Arguments
- `-o, --stdout` - Prints the result of each command to the terminal
- `-t, --times <TIMES>` - How many times the command should be ran
- `-h, --help` - Prints help

## Usage
### Timing execution of command being ran once
`timers <command>`

### Timing execution of command ran multiple times
`timers -t 1000 <command>`

### Printing output of commands to stdout
`timers -t 1000 -o <command>`
