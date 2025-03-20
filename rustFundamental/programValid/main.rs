fn main() {
    // outer scope executes 2nd
    let x: i32 = 10;
    let y: i32 = 5;
    {
        // inner scope executes 1st
        let x: i32 = 15;
        let y: i32 = 10;
        println!("The value of x is {} and the value of y is {}", x, y);
    }
    println!("The value of x is {} and the value of y is {}", x, y);
}
