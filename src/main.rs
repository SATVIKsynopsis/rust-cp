use libc::*;
use std::env;
use std::ffi::CString;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 3 {
        println!("usage: cp source_file destination_file");
        return;
    }

    let source_file_name = CString::new(arguments[1].as_str()).unwrap();

    let destination_file_name = CString::new(arguments[2].as_str()).unwrap();

    unsafe {
        let source_file_descriptor = open(source_file_name.as_ptr(), O_RDONLY);

        if source_file_descriptor < 0 {
            println!("error opening source file");
            return;
        }

        let destination_file_descriptor =
            open(destination_file_name.as_ptr(), O_WRONLY | O_CREAT, 0o644);

        if destination_file_descriptor < 0 {
            println!("Error creating destination file");
            close(source_file_descriptor);
            return;
        }

        let mut temporary_buffer = [0u8; 100];

        loop {
            let number_of_bytes_read = read(
                source_file_descriptor,
                temporary_buffer.as_mut_ptr() as *mut _,
                100,
            );

            if number_of_bytes_read == 0 {
                break;
            }

            if number_of_bytes_read < 0 {
                println!("Error reading file");
                break;
            }

            write(
                destination_file_descriptor,
                temporary_buffer.as_ptr() as *const _,
                number_of_bytes_read as usize,
            );
        }

        close(source_file_descriptor);
        close(destination_file_descriptor);
    }
}
