fn main() {
    println!("Hello, world!");
    alt();
    alt_with_params(75);
    alt_labeled_measurement(6, 'h');
    //y is a statement, and the code inside the block is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    let x = five();
    println!("X equals {}",x);

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn alt() {
    println!("This is another function");
}

fn alt_with_params(x: i32){
    println!("The value of x is: {x}");
}

fn alt_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//Statements are "void"(no return) functions. Expressions return values

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}