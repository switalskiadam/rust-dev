# Building via Cargo
## cargo new
In your terminal type
>$ cargo new folderdir

This will create a new directory and 
- a src folder with a default main.rs file
- cargo.toml file
- it also executes a git init unless you are already within a git repo
## Cargo.toml
>[package]  
>name = "hello_cargo"  
>version = "0.1.0"  
>edition = "2021"  
>  
>  See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
>  
>[dependencies]  
### package
Name, version and edition are all used to compile the code
### dependencies
Here you can list any crate dependencies for your project
- crates are packages of rust code

## main.rs
> fn main() {  
> &nbsp;&nbsp;&nbsp;&nbsp;println!("Hello, world!")  
>}

This is the default rust file that is generated from cargo new
The file is generated within the src directory
- All source files should live within this directory
- Top level directory is for README files, source files, configs, etc.
## cargo build
This command creates:
- a new target directory
    - many new files are generated, the newly created .exe is within the debug directory
- cargo.lock file
    - this keeps track of the exact dependencies of the project 
## cargo run
Rather than type the entire directory to run the executable you can simply use:
> cargo run
## cargo check
This function can simply be used to see if everything compiles correctly but doesn't create an executable
> cargo check
- We use this because it is faster than cargo build
- Beneficial if you continually check code during development
- Many will use cargo check to make sure code compiles periodically and when the project is complete they will run cargo build
## releasing a project
When the project is done we can run:
> cargo build --release
This will allow for your code to be optimized and run much faster than the version in debug
- debug exes compile quickly but run slower
- release exes compile slower but run faster
