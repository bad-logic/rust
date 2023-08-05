//-----------------------------------
//      Box Smart Pointers
//-----------------------------------

// enum List{ //recursive type `List` has infinite size
//     Cons(i32,List), 
//     Nil
// }

#[derive(Debug)]
enum List{ 
    Cons(i32,Box<List>), 
    Nil
}

use List::{Cons,Nil}; // moving scope to parent scope 
fn main(){
    // let list:List = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
    // since scope is moved up, can be rewritten as
    // let list:List = Cons(1, Cons(2, Cons(3, Nil)));
    let list:List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list {:?}",list);
}