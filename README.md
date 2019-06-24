# ICFP Programming Contest 2019
## Worker-Wrappers against Bit Rot

https://icfpcontest2019.github.io/download/specification-v1.pdf

This is a 99.9% rust implementation with one small sh script to zip up and submit solutions.

## To build with rust installed :
* cd to the src dir and then run cargo build
* to build and run you can run cargo run FILE...
* Use the -s switch to select solver
* Use the -h switch for help/info

### For example:
* cargo run --release -- problems/prob-001.desc
* cargo run --release -- problems/prob-*.desc
* cargo run --release -- -s eager problems/prob-*.desc

## Solvers
* boko_retry
* boko
* right
* spiral_right
* directed
* dis
* eager

Thanks for the great contest!!
