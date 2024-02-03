fn main() {
    let x = 5;
    println!("x = {}",x);
    //x=6       Doesn't run because is not labeled mut
    let mut y =1;
    println!("y = {}",y);
    y=8;
    println!("y = {}",y);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("This is a constant: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
