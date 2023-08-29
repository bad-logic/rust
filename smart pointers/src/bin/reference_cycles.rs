//------------------------------------
//      Reference Cycles
//------------------------------------

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node{
    // next: Option<Rc<RefCell<Node>>>,
    next: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node{
    fn drop(&mut self){
        println!("Dropping {:?}",self);
    }
}

fn main(){

    // let a = Rc::new(RefCell::new(Node{next:None}));
    // println!("a count: {:?}",Rc::strong_count(&a));

    // let b = Rc::new(RefCell::new(Node{next:Some(Rc::clone(&a))}));
    // println!("a count: {:?}",Rc::strong_count(&a));
    // println!("b count: {:?}",Rc::strong_count(&b));

    // let c = Rc::new(RefCell::new(Node{next:Some(Rc::clone(&b))}));

    // // creating reference cycle
    // (*a).borrow_mut().next = Some(Rc::clone(&c));
    // println!("After creating cycle: \na count: {:?}",Rc::strong_count(&a));
    // println!("b count: {:?}",Rc::strong_count(&b));
    // println!("c count: {:?}",Rc::strong_count(&c));


    // println!("a {:?}",a);// stack overflow, memory leakage


    // solution: Weak rc pointer

    let a = Rc::new(RefCell::new(Node{next:None}));
    println!("a count: {:?}, a weak count {:?}",Rc::strong_count(&a),Rc::weak_count(&a));

    let b = Rc::new(RefCell::new(Node{next:Some(Rc::downgrade(&a))}));
    println!("a count: {:?}, a weak count {:?}",Rc::strong_count(&a),Rc::weak_count(&a));
    println!("b count: {:?}, b weak count {:?}",Rc::strong_count(&b),Rc::weak_count(&b));

    let c = Rc::new(RefCell::new(Node{next:Some(Rc::downgrade(&b))}));

    (*a).borrow_mut().next = Some(Rc::downgrade(&c));
    println!("After creating cycle: \na count: {:?}, a weak count {:?}",Rc::strong_count(&a),Rc::weak_count(&a));
    println!("b count: {:?}, b weak count {:?}",Rc::strong_count(&b),Rc::weak_count(&b));
    println!("c count: {:?}, c weak count {:?} ",Rc::strong_count(&c),Rc::weak_count(&c));

    println!("a {:?}",a); // no memory leakage


}