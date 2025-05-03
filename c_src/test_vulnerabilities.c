#include "buffer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Test functions for each type of vulnerability
void test_buffer_overflow() {
    printf("\n=== Buffer Overflow Test 1 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer with capacity 10\n");
    
    // Append data that fits within capacity
    buffer_append(buffer, "Hello", 5);
    buffer_print(buffer);
    
    // Overflow: Append more data than the remaining capacity
    printf("Attempting to append 10 more bytes when only 5 bytes remain...\n");
    buffer_append(buffer, " World!!!", 10);
    
    // This print may show corrupted data or crash
    printf("After overflow: ");
    buffer_print(buffer);
    
    buffer_free(buffer);
    printf("Test completed\n");
}

void test_double_free() {
    printf("\n=== Double Free Test 2 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer\n");
    
    // Free immediately
    printf("Freeing buffer first time...\n");
    buffer_free(buffer);
    
    // Wait a bit to potentially trigger different behavior
    printf("Waiting briefly...\n");
    for (volatile int i = 0; i < 1000000; i++) {}
    
    // Double free after some time
    printf("Attempting to free buffer second time after delay...\n");
    buffer_double_free(buffer);  // This will cause a double-free error
    
    printf("Test completed\n");  // We may not reach here
}

void test_use_after_freed() {
    printf("\n=== Use After Free Test 2 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer\n");
    
    buffer_append(buffer, "Hello", 5);
    buffer_print(buffer);
    
    // Free the buffer
    printf("Freeing buffer...\n");
    buffer_free(buffer);
    
    // Use after free: try to append more data
    printf("Attempting to append data after free...\n");
    buffer_append(buffer, " World", 6);  // Writing to freed memory
    
    // Try to print after use-after-free
    //printf("After use-after-free append: ");
    //buffer_print(buffer);  // This will likely crash
    
    printf("Test completed\n");  // We may not reach here
}
void test_buffer_over_read() {
    printf("\n=== Buffer Over-Read Test 1 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer\n");
    
    buffer_append(buffer, "Hello", 5);
    buffer_print(buffer);
    
    // Try to read beyond the valid data
    printf("Attempting to read beyond buffer size...\n");
    printf("Valid buffer size: %zu bytes\n", buffer->size);
    printf("Attempting to read 10 bytes from buffer...\n");
    
    // Intentional buffer over-read
    for (size_t i = 0; i < 15; i++) {
        char c = buffer->data[i];
        printf("Byte at position %zu: %c (%d)\n", i, (c >= 32 && c <= 126) ? c : '.', c);
    }
}

int main() {
    printf("Starting vulnerability tests for buffer manager\n");
    printf("WARNING: These tests will crash due to intentional memory vulnerabilities\n");
    printf("Each test should be run separately by uncommenting one at a time\n\n");
    
    // Uncomment one test at a time to observe behavior
    
    test_buffer_overflow();
    test_buffer_over_read();
    test_double_free();
    test_use_after_freed();

    printf("\nAll tests completed (if you see this, some tests didn't crash as expected)\n");
    return 0;
}