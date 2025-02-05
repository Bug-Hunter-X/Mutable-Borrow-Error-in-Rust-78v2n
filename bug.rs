fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &mut x; //This will cause an error because x is already mutably borrowed by y
    *z = 12; 
}