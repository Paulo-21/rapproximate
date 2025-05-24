# RARIPOTER
An reimplementation of ARIPOTER, an approximate reasoning solver for ICCMA tasks.

## Compiling
To compile ARIPOTER, you need to install Rust: https://www.rust-lang.org/

Then:
```
cargo build -r
```
The binarie aripoter will be in the directory `target/release` when compiling with "-r" option.

## Running
```
Usage: aripoter [OPTIONS]

Options:

  -a, --argument <ARGUMENT>    Quary argument for credulous and skeptical acceptance

  -f, --input_AF <INPUT_AF>    Path of the file containing the AF

  -p, --task <TASK>            A computational problem supported by the solver (e.g. DC-CO, DS-PR)

      --problems               Prints the supported computational problems and exits

      --heuristic <HEURISTIC>   Avalaible options : harper, inout, hcat, noselfatt, card, maxb, counting, iccma2025 heuristic is the default heuristic

  -v, --verbose                Print details of the execution time of each part of the solution

                               " to parse the file ; to solve the grounded extention ; to solve with an heuristic ; the result "

  -h, --help                   Print help

  -V, --version                Print version
```