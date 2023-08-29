use std::cell::RefCell;

fn main(){

    // let x = 32;
    // mutable reference of immutable variable
    // is not allowed by default in rust
    // let x1 = &mut x;


    let a = RefCell::new(10);
    // mutable reference of immutable variable allowed on RefCell pointer values
    let mut b = a.borrow_mut();
    
    *b = 15;
    
    
    println!("{}",b);
    println!("{:?}",a);
    
    drop(b);
    println!("{:?}",a);

    *a.borrow_mut() = 30;
    println!("{:?}",a);


}