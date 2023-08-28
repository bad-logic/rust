//-----------------------------------------------------
// 
//          Displaying Participant of an online meeting
//              - Description
//                  - Retrieving list of paginated view of the participants in an online meeting
//                  
//  
//          - Tools
//              - BST + Stack
// 
//-----------------------------------------------------

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug,Default,PartialEq,Eq,Clone)]
struct Node {
    left: Link,
    value: String,
    right: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(val:String) ->Self{
        Self { left: None, value:val, right: None }
    }    

    fn insert(&mut self, val:String){
        if val > self.value{
            match &self.right {
                Some(right) => {
                    right.borrow_mut().insert(val);
                },
                None => {
                    self.right = Some(Rc::new(RefCell::new(Node::new(val))))
                }
            }
        }else{
            match &self.left {
                Some(left) => {
                    left.borrow_mut().insert(val);
                },
                None => {
                    self.left = Some(Rc::new(RefCell::new(Node::new(val))))
                }
            }
        }
    }
}

#[derive(Debug,Default,PartialEq,Eq)]
struct BinarySearchTree {
    root: Node
}

impl BinarySearchTree {
    fn new(val:String)->Self{
        Self { root: Node::new(val) }
    }

    fn insert(&mut self,val:String) {
        self.root.insert(val);
    }
}

#[derive(Debug)]
struct DisplayLobby{
    stack: Vec<Rc<RefCell<Node>>>
}

impl DisplayLobby {
    fn new(root: Option<Rc<RefCell<Node>>>)-> Self{
        let mut stack = Vec::new();
        Self::push_all_left(root,&mut stack);
        DisplayLobby { stack }
    }

    fn push_all_left(mut p:Option<Rc<RefCell<Node>>>, stack: &mut Vec<Rc<RefCell<Node>>>){
        while let Some(link) = p.clone() {
            stack.push(p.clone().unwrap());
            p = link.borrow().left.clone();
        }
    }

    fn next_name(&mut self) -> String{
        let node = self.stack.pop().unwrap();
        let name = &node.borrow().value;

        let next_node = node.borrow().right.clone();

        Self::push_all_left(next_node, &mut self.stack);
        name.to_string()
    }

    fn next_page(&mut self) -> Vec<String> {
        let mut resultant_names = Vec::new();
        for i in 0..10{
            if !self.stack.is_empty(){
                resultant_names.push(self.next_name());
            }else{
                break;
            }
        }
        resultant_names
    }
}

fn main(){
     
   let mut bst = BinarySearchTree::new("Monica".to_string());
   let names_list = vec![
    "Chandler",
    "Ross",
    "Phoebe",
    "Rachel",
    "Joey",
    "Ursula",
    "Mike",
    "Gunther",
    "Janice",
    "Ben",
    "Emma",
    "Bonnie",
    "Kathy",
    "Mona",
    "Julie",
    "Carol",
    "Emily",
    "Susan",
    "Amy",
    "Jill",
    "Pete",
    "Judy",
    "Jack",
    "Frank",
    "Nora",
    "Janine",
    "Estelle",
   ];

   for name in names_list {
    bst.insert(name.to_string());
   }

   println!("{:?}",bst);
   
   println!();

   let mut display = DisplayLobby::new(Some(Rc::new(RefCell::new(bst.root))));

   println!("display loggy {:?}",display);
   println!("Participants on the first page {:?}",display.next_page());

   println!("Participants on the second page {:?}",display.next_page());

   println!("Participants on the third page {:?}",display.next_page());
   println!("Participants on the fourth page {:?}",display.next_page());


}