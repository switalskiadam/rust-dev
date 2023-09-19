# Control flow
- Controlling the code through a variety of expressions can allow us to repeat code until a criteria is met or only execute certain code when criteria is met
## if conditions
- An if condition allows you to "branch" your code based on certain conditions
    - i.e. if this condition is met, run this code else run the other code
> fn main() {  
>&nbsp;&nbsp;&nbsp;&nbsp;if 'condition is true' {  
>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;'do this';   
>&nbsp;&nbsp;&nbsp;&nbsp;} else {  
>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;'do that';  
>&nbsp;&nbsp;&nbsp;&nbsp;}   
}
- The condition in if statements must be boolean
    - Rust will not try to convert non-Boolean types to boolean to evalute the if statement
- If statements can have more than 2 "branches" by using else if statements
- Rust will evaluate the branches in order until the condition specified returns as true
- Using too many else ifs can clutter your code and make it hard to read so if you are using more than one you should refactor your code
- Since if is an expression, we can use it on the right of a let statement to assign the outcome to a variable
    - When assigning variables via this methodology, we need to make sure that both branches evaluate to the same data type or rust will throw an error
