//-----------------------------------------------------
// 
//          Most Recently used product
//              - Description
//                  - A business is interesting in knowing the products that has been
//                       purchased most recently by a customer
//  
//          - Tools
//              - Hashmaps + Doubly Link List
// 
//-----------------------------------------------------

use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Node{
    element: i32,
    next: Pointer,
    prev: Pointer
}

impl Node{
    fn new(x:i32)-> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Node{
            prev:None,
            element: x,
            next: None,
        }))
    }
}

type Pointer = Option<Rc<RefCell<Node>>>;

#[derive(Default,Debug,Clone)]
struct List{
    head: Pointer,
    tail: Pointer
}

impl List {
    fn new() -> List{
        List{
            head: None,
            tail: None
        }
    }

    fn push_back(&mut self,element:i32)-> Pointer{
        let new_tail = Node::new(element);
        match self.tail.take(){
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            },
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.tail.clone()
    }
    
    fn remove_front(&mut self)-> Option<Pointer>{
        if self.head.is_none(){
            println!("the list is empty, cannot remove");
        }else{
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take(){
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take();
                        self.head = Some(new_head);
                        self.head.clone()
                    },
                    None => {
                        self.tail.take();
                        println!("List is empty after removal");
                        None
                    }
                }
            });
        }
        Some(self.head.clone())
    }

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>){
        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));
        match ( prev, next ){
            (None, None) => {
                // already a tail
            },
            (Some(_),None) => {
                // already a tail
            },
            (None,Some(next)) =>{
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;
                self.head = Some(next.clone());
                
                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                self.tail = Some(node.clone());
                
            },
            (some(prev),Some(next))=>{
                node.borrow_mut().next = None;
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());

            }
        }
    }

}

#[derive(Debug)]
struct MRP_ITEM { // most recently purchased MRP
    map: HashMap<i32, Rc<RefCell<Node>>>,
    item_list: List,
    size: i32,
    capacity: i32,
}

impl  MRP_ITEM {
    fn new(capacity:i32) -> Self{
        Self{
            map:HashMap::new(),
            item_list: List::new(),
            size: 0,
            capacity
        }
    }

    fn purchase (&mut self,prod_id:i32){
        if let Some(node) = self.map.get(&prod_id){
            self.item_list.move_to_tail(node);
        }else {
            if self.size >= self.capacity{
                let prev_head = self.item_list.remove_front().unwrap();
                self.map.remove(&prev_head.unwrap().borrow().element);
            }
            let node = self.item_list.push_back(prod_id).unwrap();
            self.map.insert(prod_id,node);
            self.size += 1;
        }
    }

    fn print(&self){
        let mut traversal = self.item_list.head.clone();
        while !traversal.is_none(){
            let temp = traversal.clone().unwrap();
            print!("{}",temp.borrow().element);
            traversal = temp.borrow().next.clone();
        }
        println!();
    }
}

fn main(){
    let mut items_list = MRP_ITEM::new(3);
    items_list.purchase(10);
    items_list.print();
   
}