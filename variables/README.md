# Variables and mutability
- By default in rust, variables are immutable or unable to be changed
- The immutability forces you to program in a safe manner
> let x = 5  
    
    - This will result in an immutable variable
    - Attempting to change the value of x later in the code will result in an error during compilation of the code
    - If we want the ability to change the value through our code we must specify that the variable is mutable by adding that to the line that sets the variable
> let mut x = 5
- This also will tell other readers know that part of the code later will change the value

## Constants
- Very simliar to immutable variables, constants are not allowed to change
- Constants are always immutable and unable to become mutable
- Constants are declared by using const instead of let
    - The data type MUST be annotated
    - They cannot be a result of a value computed at run time
- E.g.
> const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;  

    - Rusts naming convention for constants is to use all upper case characters with underscores for spaces
- Constants are valid for the entire time the program runs, within the scope they were declared
- Relevant uses of a constant
    - Maximum number of points a player can earn
    - Speed of light
## Shadowing
- Shadowing is the act of taking an existing variables name and setting it to a new value
- Shadowing is different from declaring a variable as mut, let has to be used to shadow or we will run into a compile-time error 
- This allows us to transform the variable but leave it immutable after changing it
    - This includes data type changes

