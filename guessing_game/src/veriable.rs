fn main() {

    // mutable variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let y = 6;
    let y = y + 1;
    println!("The value of y is: {y}");
   

    // loop with reverse
    let a = [10, 20, 30, 40, 50];
    
        for number in a.iter() {
            println!("{number}");
        }
        println!("LIFTOFF!!!");
    

}
    
