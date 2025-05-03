pub struct BufferR {
    pub data: Vec<u8>,
}

impl BufferR {
    // Create a new buffer with initial capacity
    pub fn new(initial_capacity: usize) -> Self {
        BufferR {
            data: Vec::with_capacity(initial_capacity),
        }
    }

    // Append data to the buffer - safe by default
    pub fn append(&mut self, input_data: &[u8]) {
        self.data.extend(input_data);
    }

    // Other methods remain the same...
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }
    // Method to get the current buffer capacity
    pub fn get_capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn print(&self) {
        println!("Buffer content ({} bytes): ", self.data.len());
        for byte in &self.data {
            print!("{:02x} ", byte);
        }
        println!();
    }
}

#[cfg(test)]

mod tests {
    use std::string;
    use std::{fmt::Error, num::ParseIntError};

    use crate::BufferR;
    #[test]
    fn rust_buffer_overflow() {
        println!("\n=== Buffer Overflow Test (Rust Implementation) ===");
        
        // Create a small buffer
        let mut buffer = BufferR::new(10);
        println!("Created buffer with capacity 10");
        
        // Add data within capacity
        buffer.append(b"Hello");
        println!("Added 5 bytes");
        println!("Buffer capacity: {:?}", buffer.get_capacity());
        buffer.print();
        
        // Try to add data exceeding capacity
        println!("Attempting to add 59 more bytes (exceeds initial capacity)...");
        buffer.append(b"This is a longer string I want to input to Rust buffer data");
        
        // Rust Vec will automatically resize - no overflow!
        println!("Rust safely handled the potential overflow by resizing the buffer");
        buffer.print();
        
        assert!(buffer.get_size() == 64);
        print!("Buffer capacity after added exceeded data: {}", buffer.get_capacity());
        println!("\nTest passed: Buffer correctly contains all 64 bytes");
        println!("Rust drops buffer memory once it goes out of scope")
    }
    #[test]
    #[should_panic]
    fn rust_double_free() {
        println!("\n=== Double free Test (Rust Implementation) ===");
        {
        // Create a small buffer
        let mut buffer = BufferR::new(10);
        println!("Created buffer with capacity 10");
        
        // Add data within capacity
        buffer.append(b"Hello");
        println!("Added 5 bytes");
        buffer.print();
        } //
        println!("buffer out of scope here");
        println!("Rust prevent double free and use after free, by");
        println!("automatically drop buffer memory, once it out of scope");

        print!("\n");
        println!("Introduce double free here using panic mode if undefined behavior or memory bugs");
        println!("error occurs at compile-time to prevent memory issues or vulnerabilies");
        //drop(buffer); //
    }

    #[test]
    #[should_panic]
    fn rust_over_read() {
        println!("\n=== Over-Read Test (Unsafe Rust Implementation) ===");
        let mut buffer = BufferR::new(10);
        println!("Created buffer with capacity 10");
        buffer.append(b"Hello"); // 5 bytes
        println!("Added 5 bytes");
        buffer.print();
        println!("\nAttempting to read at index 5 or more (out of bounds)");
        // Intentional buffer over-read
        for i in 0..15 {
            let byte = buffer.get_data()[i];
            let c = byte as u8; // cast i8 to u8
            // attempt to access raw data over the
            println!("Byte at position {}: {} ({})", 
                    i, if c >= 32 && c <= 126 {c as char} else {'.'},
                c);
        }
        // prevent memory issues with panic mode
    }

    #[test]
    #[should_panic]
    fn rust_use_after_free() {
        {
            // Create a small buffer
            let mut buffer = BufferR::new(10);
            println!("Created buffer with capacity 10");
            
            // Add data within capacity
            buffer.append(b"Hello");
            println!("Added 5 bytes");
            buffer.print();
        } //
        
        println!("Try to append buffer after freed");
        //buffer.append(b"Word");
        //if let e = string::from
    }

    use crate::LowLevelBuffer;
    #[test]
    fn rust_low_buffer_overflow() {
        println!("\n=== Buffer Overflow Test (Rust Implementation) ===");
        
        // Create a small buffer
        let mut low_buffer = LowLevelBuffer::new(10);
        println!("Created buffer with capacity 10");
        
        // Add data within capacity
        low_buffer.append(b"Hello");
        println!("Added 5 bytes");
        //low_buffer.print();
        
        // Try to add data exceeding capacity
        println!("Attempting to add 59 more bytes (exceeds initial capacity)...");
        low_buffer.append(b"This is a longer string I want to input to Rust buffer data");
        
    }
}

pub struct LowLevelBuffer {
    data: *mut u8,
    size: usize,
    capacity: usize,
}

impl LowLevelBuffer {
    pub fn new(capacity: usize) -> Self {
        // Use Rust's layout and allocator APIs
        let layout = std::alloc::Layout::array::<u8>(capacity).unwrap();
        let data = unsafe { std::alloc::alloc(layout) };
        
        if data.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        
        LowLevelBuffer {
            data,
            size: 0,
            capacity,
        }
    }

    pub fn append(&mut self, new_data: &[u8]) -> bool {
        if self.size + new_data.len() > self.capacity {
            // Reallocate with more capacity
            let new_capacity = self.capacity * 2;
            let old_layout = std::alloc::Layout::array::<u8>(self.capacity).unwrap();
            
            unsafe {
                let new_ptr = std::alloc::realloc(
                    self.data,
                    old_layout,
                    new_capacity
                );
                
                if new_ptr.is_null() {
                    return false;
                }
                
                self.data = new_ptr;
                self.capacity = new_capacity;
            }
        }
        
        // Copy the new data
        unsafe {
            std::ptr::copy_nonoverlapping(
                new_data.as_ptr(),
                self.data.add(self.size),
                new_data.len()
            );
        }
        
        self.size += new_data.len();
        true
    }
}

impl Drop for LowLevelBuffer {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<u8>(self.capacity).unwrap();
                std::alloc::dealloc(self.data, layout);
            }
        }
    }
}