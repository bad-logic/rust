//-----------------------------------------------
//      Reference Counting (RC) Smart Pointer
//          - creates another owner of the data
//          - increases reference count of the data
//          - allows multiple owner of the data using clone function
//          - dropped when there are no owners of the data
//-----------------------------------------------

use std::rc::Rc;

enum List{
    // Cons(i32,Box<List>),
    Cons(i32,Rc<List>),
    Nil
}
use List::{Cons,Nil};

fn main(){
    // let a = Cons(1,Box::new(Cons(2,Box::new(Nil))));
    // let b = Cons(3,Box::new(a));// ownership of `a`'s value changed to `b`
    // let c = Cons(4,Box::new(a)); // error value `a` used after move


    // Rc::clone does not clone and store the data,
    // it only increases the reference count
    // Rc::clone tells rust that we are creating another owner which will point to same data
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("Reference count of a = {}",Rc::strong_count(&a));

    {
        let b = Rc::new(Cons(3, Rc::clone(&a)));
        println!("Reference count of a = {}",Rc::strong_count(&a));
        
        let c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("Reference count of a = {}",Rc::strong_count(&a));
    }

    println!("Reference count of a = {}",Rc::strong_count(&a));


}