# rustprj

To compile and run the code, use the cargo run command in the project's directory:
$ cargo run

This will compile and run the code, and you can provide the input data through the standard input. For example:

$ cargo run
Process1
Recent file system events for process Process1 (0):
1 - read
2 - write
5 - delete

Recent child process events for process Process1 (0):
3 - 1
4 - 2

Recent thread events for process Process1 (0):
6 - 1
7 - 2

The code will read the input data from standard input, parse it into a Process struct, and print the output for the given process name or id.
