# Loops
- There are 3 kinds of loops
    - loop
    - while
    - for
## loop
- loop while tell the program to continually run the block of code until you explicitly tell it to stop
    - reminder: crtl + c can be ran in the terminal to manually stop your program or infinite loops
- Using the 'break' keyword you can tell the program to exit the loop
- We can also use the keyword 'continue' to skip any remaining code in the loop and start at the beginning
- As code can get complex, such as loops within loops, it is best practice to label the loop to allow for better control flow through break and continue
    - see function loop_la_loop
> 'loop_label: loop {...}

## while
- Programs often need to evaluate a condition within a loop, when the condition is no longer true, we have the program break
- This can be accomplished through use of if, else, loop, and break
- Since this is so common, we can leverage the while keyword which handles the logic for you
- While is a loop that will run until the condition you set is no longer true and once no longer true, it will break out of the while loop
    - see function while_n_out
- We can use while to also iterate through elements of an array
    - see function array_loop