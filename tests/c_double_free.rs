use std::ffi::CString;

use crate::bindings::{
    buffer_init, buffer_append,
    buffer_free, buffer_double_free,
    buffer_data, buffer_size,
};
#[allow(unused_imports)]
use safemem::BufferR;
mod bindings;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn c_test_double_free() {
        unsafe {
            let buffer = buffer_init(10);
            println!("C created buffer with initial capacity of 10 bytes");

            // Append data that fits
            let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
            buffer_append(&mut *buffer, c_string.as_ptr(), 5);
            println!("Buffer size: {}, buffer data: {:?}", 
            buffer_size(buffer), buffer_data(buffer));
            
            // Free immediately
            println!("C freeing buffer first time...");
            buffer_free(buffer);
            
            // Wait a bit
            println!("Waiting briefly...");
            std::thread::sleep(std::time::Duration::from_millis(10));
            
            // Double free after some time
            println!("C attempting to free buffer second time after delay...");
            buffer_double_free(buffer); // This will cause a double-free error
            
            println!("C Test completed, C allow double free the heap memory");
        }
    }

}