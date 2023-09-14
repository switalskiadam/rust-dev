# Functions
- Functions are prevalent in rust
- main() is the entry point for most programs in rust
- Snake case is the conventional style for functions and variable names
- Functions are defined by fn followed by a function name and a set of parentheses
- Functions used within the main() function can be defined before or after the main() function
    - Rust doesn't care where you define functions as long as they defined in scope that can be seen by the caller
## Parameters
- Parameters are special variables that are a part of a function
- Specifying the data type is required
    - This allows for better error messages
    - Doesn't need to use the type allocation later in the compiler to figure out what type you mean
> fn some_function(x: i32) {  
> &nbsp;&nbsp;&nbsp;&nbsp;...  
>}

- When defining multiple parameters they are separated by a comma

## Statements and Expressions
- Statements 
    - Instructions that perform an action but doesn't return a value
    - Therefore, you cannot assign a let statement to another variable
    > let x = (let y = 6);
        
        - This throws an error because let y = 6 doesn't return a value
- Expressions
    - Evaluate a result value
    - Examples
        - Math operations, 5 + 6
        - Calling a function
        - Calling a macro
        - A new scoped block 
    - Do not include a ending semicolons
        - Adding a semicolon turns the expression into a statement
        - This will return no value

## Functions with return values
- Functions can be written so that a value is returned when the function is called
- This is done by using an arrow -> and setting the data type
> fn five() -> i32 {  
>&nbsp;&nbsp;&nbsp;&nbsp;5  
>}  
>fn plus_one(x: i32) -> i32 {  
>&nbsp;&nbsp;&nbsp;&nbsp;x + 1  
>}

    - note that there is no semicolon behind the 5 or other values preventing it from becoming a statement