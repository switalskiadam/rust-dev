# Creating a simple function in Rust
## Structure
> fn main() {  
>  
>}
## Notes
- Main is always the first code that is executed in any Rust program
- Rust style uses 4 spaces for indentation, not tab
- println! is a macro 
    - ! indicates calling a macro
    - no ! would call a function
    - ends with a ;, this tells the program to stop and execute the next line of code
### Compiling vs Running
- rustc compiles your code and creates an executable
> $ rustc sourcefile.rs 
- to run the rust script you need to then call the exe in the terminal (cmd)
> $ .\sourcefile.exe 