In this project, I implemented a basic version of the Linux cp command using Rust.
The goal of this project was to understand how file copying works at the operating system level and how system calls can be used in Rust with the `libc` crate.

Instead of using Rust’s built-in file handling functions, I directly used Linux system calls like `open`, `read`, `write`, and `close`.

The program first reads the command line arguments using `env::args()` to get the source and destination file names provided by the user. It then converts these file names into `CString` format because Linux system calls require C style strings. Inside an unsafe block, the program opens the source file in read only mode and creates the destination file in write mode using the open system call. After that, it uses a loop where it repeatedly reads a small chunk of data from the source file into a buffer using the read system call and immediately writes the same data into the destination file using the write system call. This process continues until the end of the source file is reached. Finally, the program closes both the source and destination files using the close system call to free system resources.
