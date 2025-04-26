use std::ffi::CString;

use crate::bindings::{
    buffer_init, buffer_append,
    buffer_capacity,
    buffer_size,
};
#[allow(unused_imports)]
use safemem::BufferR;
mod bindings;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_test_buffer_overflow() {
        unsafe {
            let buffer = buffer_init(10);
            println!("Created buffer with capacity 10 bytes");
            
            // Append data that fits
            println!("Adding 'Hello' word, a 5 bytes to buffer");
            let c_string = CString::new("Hello").unwrap(); // CString adds the null terminator auto
            buffer_append(&mut *buffer, c_string.as_ptr(), 5);
            //println!("Buffer after added 'Hello'");
            println!("Buffer size: {}, buffer capacity: {:?} after added 'Hello'", 
            buffer_size(buffer), buffer_capacity(buffer));
            
            // Append data that exceeds capacity (will cause overflow)
            println!("Attempting to append 10 more bytes when only 5 bytes remain...");
            let append_c_string = CString::new(" Wolrd!!!").unwrap();
            buffer_append(buffer, append_c_string.as_ptr(), 10);
            
            // This might crash or show corrupted data
            println!("Buffer size: {}, buffer capacity: {:?} after tried to overflow", 
                        buffer_size(buffer), buffer_capacity(buffer));
            
            //buffer_free(buffer);
            println!("C Test completed");
        }
}
}

