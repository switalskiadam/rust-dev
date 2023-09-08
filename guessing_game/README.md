# Guessing game
## Importing packages
> use std::io;

- By using the use command (outside of the function) you can bring in libraries to work with. From the standard library (std) we are importing the input/output (io) library.
- By default, every rust program has a standard library that is always brought into scope. This is called the prelude.

## Breaking down the function
- All functions use the syntax:
> fn FUNC () {  
>   
>}  

- Values within the () indicate function parameters

## Variables
> let mut guess = String::new();
- The let statement declares a variable
- The mut portion designates that the variable is mutable or able to be changed
    - Removing mut binds the variable to the value allocated
    > let apples = 5; // immutable  
    > let mut bananas = 5; // mutable
    - The syntax // is how you comment out lines
- let mut guess introduces a new variable called guess
- The equal sign indicates what the new variable should be set to
- String is the data type assigned to the variable
- :: depicts an associated function
    - ::new states that the function 'new' is associated to the String data type
- All together the line of code says to create a new mutable variable called guess that has a String data type

## Recieving input
> io::stdin()  
> &nbsp;&nbsp;&nbsp;&nbsp;.read_line(&mut guess)
- First we call the stdin() function from the io module
    - Optionally, if we didn't declare use std::io at the beginning we can still import the module by using std::io::stdin
- The next line .read_line(&mut guess) calls the read_line method to capture the input from the stdin() function
    - We pass the &mut guess as an arugment to tell the method to store the input to the designated variable, guess
        - The strings argument must be mutable so that the method can change the value
- The & sign is used as a reference. This allows your code to reference one piece of data without needing to copy in memory multiple times

## Error handling
> .expect("Failed to read line.");
- The overall code could have been written in one line, but this can become difficult as more methods are chained
> io::stdin().read_line(&mut guess).expect("Failed to read line");  

vs.
> io::stdin()  
>&nbsp;&nbsp;&nbsp;&nbsp;.read_line(&mut guess)  
>&nbsp;&nbsp;&nbsp;&nbsp;.expect("Failed to read line");  

- The read_line method also returns a Result value. 
- Results are called enumerations or enum which is a type that can be in one of multiple states. 
- Each possible state is called a variant
- Result is used to help with error handling
- Result can have two options here: Ok and Err
    - Ok indicates that the operation was a success
    - Err indicates the operation failed
        - Err will contain information on why the operation failed
    - If you don't call expect in this situation, during the compile it will give you a warning

## Printing values with placeholders
> &nbsp;&nbsp;&nbsp;&nbsp;println!("You guessed: {guess}");  
- Using the {} will used for helping print variables or other outputs
> let x = 5  
> let y = 10  
&nbsp;  
> println!("x = {x} and y + 2 = {}, y + 2);
- This will print x = 5 and y + 2 = 12

## Adding crates
- This is where cargo is really beneficial. By opening up the cargo.toml file we can specify which crates can be used within our project.
- We add rand = "0.8.5" to the toml file
> [dependencies]  
> rand = "0.8.5"
## Reproducible builds
- Because we used the command cargo build a cargo.lock file was generated
- The cargo.lock file is used to ensure that the proper versions of crates are used 
### Updating crates
- To bypass the cargo lock file you can run the command
> cargo update
- This will reference the toml file and update your crates accordingly and then write the updates to the lock file

## Generating random numbers
- First we import the rand library and the Rng package
- This allows us to call thread_rng function 
- Then we use the gen_range method to set the range of the random number function
    - The range expression we use here is start..=end and operates inclusively

## Comparing numbers
- To compare numbers we will bring in another package from the std library, cmp::Ordering
- Ordering is another enum with 3 variants, Less, Greater, Equal
- Then we use the match expression which is made up of arms, or different conditions that can be met with the resulting code that should be ran 
> match guess.cmp(&secret_number) {  
> &nbsp;&nbsp;&nbsp;&nbsp;Ordering::Less => println!("Too small!"),  <--- this is an "arm"  
> &nbsp;&nbsp;&nbsp;&nbsp;Ordering::Greater => println!("Too big!"),  
> &nbsp;&nbsp;&nbsp;&nbsp;Ordering::Equal => println!("You win!"),  
> }

## Data type mismatches
- Because we wrote:
> let mut guess = String::new()
- Rust inferred the data type to be a string
- secret_number is a number type so therefore we cannot compare a number type to a string type
- We need to convert the string type to number before the code will compile and run correctly
    - We add in this line to conver the string to number
    > let guess: u32 = guess.trim().parse().expect("Please type a number!");
    - We already have a variable defined as guess in the program but through the shadowing technique, rust allows use to reuse the variable rather than forcing us to create two variables
    - .trim() method removes any whitespace from the string
        - This also includes any escape characters
        - The user presses enter to close read_line which results in the string containing \n
    - .parse() method allows a string to be converted to another type
        - We tell Rust explicitly what data type is should be by using this syntax 
        > let guess: u32
        - The value we are comparing against should also be of the same type (u32)
        - This method will only work on characters that can logically be coverted into a number, emojis will not covert
        - Because .parse() might fail, we need to add the .except() method to handle any errors
## Loops!
- To create a loop we simply use the loop keyword 
> loop {  
>    ...  
>    }
- This creates an infinite loop
    - To force quit this loop you can enter ctrl-c in the terminal
    - Because of the .except() method used earlier you can type a string and it will cause the program to crash as well as a means to "escape" the loop
        - Not the best way to handle
    - We can set the Equal arm to cause the program to break when the correct number is guessed
    > Ordering::Equal => {  
    > &nbsp;&nbsp;&nbsp;&nbsp;println!("You win!");  
    > &nbsp;&nbsp;&nbsp;&nbsp;break;  
        
    - break here causes the program to exit the loop and complete main and end the program

## Handling invalid inputs
- Rather than having the program crash when a non-numeric value is guessed we can route our program to go back to the start of the loop on an error
- Since we know that parse will return a results that is an enum and will return 2 variants we can use the match expression and arms to tell our code what to do
- The two variants are Ok and Err
    - Ok will return with the num if parse is successful so the result we are matching with is shown as Ok(num)
    - The other match criteria is shown as Err but in order to catch ALL errors we pass in an underscore to represent that, Err(_)
        - Here we then say => continue to tell the code to stop and go back to the start of the loop