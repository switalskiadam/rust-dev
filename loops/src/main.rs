fn main() {
    //infinite_loop();
    //return_value_loop();
    //loop_la_loop();
    while_n_out();
    array_loop();
    for_score();
    while_n_out_updated();
}

fn infinite_loop() {
    loop {
        println!("again!");
    }  
}

fn return_value_loop() {
    let mut counter = 0; //setting a counter variable to keep track of what loop iteration we are on

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_la_loop() {
    let mut count = 0; // set loop counter
    'counting_up: loop { //labelled loop
        println!("count = {count}");
        let mut remaining = 10; // set remaining variable

        loop {
            println!("remaining = {remaining}"); //print remaining var
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_n_out () {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn array_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_score() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn while_n_out_updated() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}