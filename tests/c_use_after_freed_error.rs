use std::ffi::CString;

use crate::bindings::{
    buffer_init, buffer_append,
    buffer_free, buffer_size, buffer_data
};
#[allow(unused_imports)]
use safemem::BufferR;
mod bindings;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_use_after_freed_error() {
        println!("\n=== Buffer Over-Read Test 1 ===\n");
        unsafe {
            let buffer = buffer_init(10);
            println!("C created buffer with initial capacity of 10 bytes");

            // Append data that fits
            let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
            buffer_append(&mut *buffer, c_string.as_ptr(), 5);
            println!("Buffer size: {}, buffer data: {:?}", 
            buffer_size(buffer), buffer_data(buffer));
            
            // Free immediately
            println!("Free buffer first time...");
            buffer_free(buffer);
            
            // Wait a bit
            println!("Waiting briefly...");
            std::thread::sleep(std::time::Duration::from_millis(10));

            // Append data that after free
            println!("Append data to buffer after freed buffer");
            let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
            buffer_append(&mut *buffer, c_string.as_ptr(), 5);
            
            // Try to read beyond the valid data
            //println!("Attempting to read after freed buffer...\n");
            //println!("read buffer data after freed: {:?} bytes\n", buffer_data(buffer));

            };
        println!("Test completed\n");
        }
}
