//-------------------------------------
//      Ref Cell Smart Pointer
//          - represents single ownership over the data it holds
//          - enforces borrowing rule at run time
//-------------------------------------

use std::cell::RefCell;

fn main(){
    // let mut x = 50;

    // let x1 = &x;
    // let x2 = &x;

    // references rule -> mutable and immutable ref cannot co-exist in the same scope
    // let x3 = &mut x; 

    // println!("{} {}",x1,x2);

    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    // no issue in compile time
    // panics in run time since borrowing rules are checked in run time
    // let mut d = a.borrow_mut();

    drop(b);
    drop(c);
    let mut d = a.borrow_mut();
    println!("{:?}",a);
    drop(d);
    println!("{:?}",a);

}