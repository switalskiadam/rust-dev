fn main() {

    // mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let s = 5;

    let s = s + 1;

    {
        let s = s * 2;
        println!("The value of s in the inner scope is: {s}");
    }

    println!("The value of s is: {s}");

    let spaces = "   "; // returns a string
    let spaces = spaces.len(); // returns a number

}