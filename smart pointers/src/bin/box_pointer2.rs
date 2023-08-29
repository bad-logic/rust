
// #[derive(Debug)]
// enum List{ 
//     Cons(i32,Box<List>), 
//     Nil
// }

// use List::{Cons,Nil}; 


// problem with above implementation is that even though Nil takes no space in memory
// we allocate heap space for nil


#[derive(Debug)]
enum List{ 
    Cons(i32,Option<Box<List>>)
}

use List::{Cons}; 

fn main(){

    let list:List = Cons(1, Some(Box::new(Cons(2, Some(Box::new(Cons(3, None)))))));

    println!("list {:?}",list);

}