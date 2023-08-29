//-------------------------------------------------------------
//      Avoiding Allocations
//          - take 
//              - takes single argument
//              - swaps the value with the default value
//              - return the previous value with an owned type 
//          - replace
//              - takes two arguments
//              - swaps the previous value with new mentioned value
//              - return the previous value with an owned type 
//          - swap
//              - swaps the values
//-------------------------------------------------------------

use std::mem;

#[derive(Debug)]
enum Customer{
    new {name:String},
    loyal {name:String},
    rich {name:String}
}


fn promote(user:&mut Customer){
    use Customer::*;

    *user = match user{
        // Customer::new{name} => Customer::loyal {name: name.clone()}, // heap memory allocation
        // Customer::loyal{name} => Customer::rich {name: name.clone()},// heap memory allocation

        // Customer::new{name} => Customer::loyal {name: mem::take(name)},
        // Customer::loyal{name} => Customer::rich {name: mem::take(name)},

        // replace takes two arguments destination and source
        // it replaces the destination i.e. name with empty string
        // and returns the old value of name

        Customer::new{name} => Customer::loyal {name: mem::replace(name,String::new())},
        Customer::loyal{name} => Customer::rich {name: mem::replace(name,String::new())},
        Customer::rich{name} => return

    }

}

fn main(){

    let mut customer1 = Customer::new {name:"mike".to_string()};
    promote(&mut customer1);
    println!("Customer1 {:?}",customer1);

    promote(&mut customer1);
    println!("Customer1 {:?}",customer1);

    // swap variables
    let mut s1 = "Bob".to_string();
    let mut s2 = "Charles".to_string();
    // typically a third temp value is used to swap above two strings
    // which will result in allocating a memory in heap for the temp value

    // rust way
    mem::swap(&mut s1,&mut s2);
    println!("s1: {} , s2: {}",s1,s2);

}