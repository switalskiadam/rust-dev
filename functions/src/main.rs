fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    scoped_block();
    let x = five();
    println!("The value of x is: {x}");
    let y = plus_one(6);
    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn scoped_block() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
// functions to return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
