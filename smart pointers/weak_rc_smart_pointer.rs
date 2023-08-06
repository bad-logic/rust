//--------------------------------------
//  weak Rc smart pointer
//      weak pointer may or may not point to a valid resource in memory since they are weak pointer
//      and do not stop the allocation from being dropped out from memory.
//      weak pointer only increments the weak count and does not provide an own shareable
//      copy of the allocation. the upgrade function is used on a weak Rc to upgrade it to 
//      regular Rc pointer which will provide an owned copy and the rights to change the underlying
//      data 
//--------------------------------------

use std::borrow::Borrow;
use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref};

#[derive(Debug)]
struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}


fn main(){
    let leaf = Rc::new(Node{
        value:3,
        parent:RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    let branch = Rc::new(Node{
        value:5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);


}