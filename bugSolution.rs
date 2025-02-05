fn main() {
    let mut x = 5;
    { //Adding a new scope to release the borrow of x
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 12; 
    println!("x = {}", x); // prints 12
}