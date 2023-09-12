# Data types
- Every value in Rust has to have a particular data type
- This tells Rust how to interact with the value
- Two different subsets of data types
    - Scalar
    - Compound

- Rust is a statically typed language which means it needs to know all variable types at compile-time

## Scalar Type
- Represents a single value

### Integers
- Number without a fractional component

        | Length | Signed | Unsigned |
        | ----------- | ----------- | ----------- |
        | 8-bit | i8 | u8 |
        | 16-bit | i16 | u16 |
        | 32-bit | i32 | u32 |
        | 64-bit | i64 | u64 |
        | 128-bit | i128 | u128 |
        | arch | isize | usize |
- Signed v unsigned
    - Signed indicates that the value can be negative
    - Signed variables can store numbers in a range from -(2<sup>n-1</sup>) to 2<sup>n-1</sup> -1, inclusive, n is the bit number
        - i8 can store a value between -128 and 127
    - Unsigned variable can store from 0 to 2<sup>n-1</sup>-1
        - u16 can store an int from 0 to 32,767
- Integer literals can be written in any of the following forms

| Number Literal | Example
| ----------- | ----------- |
| Decimal | 98_222 | 
| Hex | 0xff | 
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte(u8 only) | b'A' |
- Integer overflow
    - If an integer exceeds the value range the program will panic and fail
    - If compiling in release mode the value will wrap, u8 value of 257 will wrap and become 1
### Floating-point numbers
- Only two options, f32 and f64
- Both are signed
- Default type is f64 
### Booleans
- Can have two values, true and false
- One byte in size
- Specified using bool
> let f: bool = false
- Booleans are commonly used in if statements
### Characters
- Char type is the most primitive alphabetic type
- Specify char literals with single quotes
    - String literals use double quotes
- 4 bytes in size
    - Can represent more than just ASCII characters
        - Japanese, Chinese, Korean, emojis are supported
## Compound Types
- Compound types are using to grouping together multiple values into one type
    - Tuples
    - Arrays
### Tuples
- Can be of multiple different data types
- Once the length is declared, it cannot change
> let tup: (i32, f64, u8) = (500, 6.4, 1);
- To reference the values of the tuple we can use destructuring to break the tuple into single parts 
> let (x, y, z) = tup;
- We can also use a . to reference the value position or index within the tuple
> let five_hundred = tup.0;  

    - Rust uses 0 base notation
### Arrays
- All elements in an array must have the same type
- We use commas to separate values within square brackets
- Arrays have a fixed length
    - Vectors another type can grow or shrink in size
- Data in arrays are stored in the stack when you don't want them stored in heap
    - Stack
        - Fixed in size, limited memory allocated to it
        - Highly efficient, simple allocation pattern
    - Heap
        - Uses dynamic memory allocation
        - Typically larger than stack
        - Used for managing objects with a longer lifetime
- Data types can be specified as well as array length
> let a: [i32; 5] = [1, 2, 3, 4, 5];
- You can also generate an array by using a similar syntax
> let a = [3; 5];  

    - This creates [3, 3, 3, 3, 3]
- You can access array elements by using a "slicer"
> let a = [1, 2, 3];
> let first = a[0];
> println!("The first value is: {first}");
    
    - out: The first value is 1

- Accessing an element outside of the limit of the array will result in rust panicking and the program stopping