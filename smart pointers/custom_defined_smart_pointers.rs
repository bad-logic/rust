//---------------------------------------------
//      Custom Defined Smart Pointer
//          - almost always implemented using struct
//          - implement deref or drop traits
//---------------------------------------------

struct MySmartPointer<T: std::fmt::Debug>{
    value: T
}

impl<T: std::fmt::Debug> MySmartPointer<T>{
    fn new(x:T)->MySmartPointer<T> {
        MySmartPointer { value: x }
    }
}

// standard library implementation of Deref trait
// trait Deref{
//     type Target: name_of_type;
//     fn deref(&self) -> &self::Target
// }
    
use std::ops::Deref;

// impl Deref for MySmartPointer{
//     type Target = i32;
//     fn deref(&self)-> &i32{
//         &self.value
//     }
// }
impl<T: std::fmt::Debug> Deref for MySmartPointer<T>{
    type Target = T;
    fn deref(&self)-> &T{
        &self.value
    }
}


// standard library implementation of Drop trait
/*
    pub trait Drop{
        fn drop(&mut self);
    }

*/

// impl Drop for MySmartPointer{
//     fn drop(&mut self){
//         println!("Dropping MySmartPointer object from memory {:?}",self.value);
//     }
// }

impl<T: std::fmt::Debug> Drop for MySmartPointer<T>{
    // this function is called when the MySmartPointer goes out of scope
    // gives back the memory allocated for that MySmartPointer
    fn drop(&mut self){
        println!("Dropping MySmartPointer object from memory {:?}",self.value);
    }
}

fn my_fn(str:&str){
    println!("The string received from the main is {}",str);
}


fn main(){
    let a = 50;
    let b = Box::new(a);
    
    println!("{}", 50 == a);
    println!("{}", 50 == *b);

   
   let sptr1 = MySmartPointer::new(a);
   let sptr2 = MySmartPointer::new(a + 3);
   let sptr3 = MySmartPointer::new(a + 6);
   let sptr4 = MySmartPointer::new(*b);

   println!("{}",a == *sptr1); // *sptr1 = *(sptr1.deref())
   // deref always return the reference to the inner value

   // drop(sptr1);

   // deref coersions in smart pointers

    let sptr = MySmartPointer::new("Bob");
    my_fn(&sptr); // &MySmartPointer -> &String -> &str


    let some_vec = MySmartPointer::new(vec![1,2,3]);

    for z in &*some_vec{
        println!("The value is {}",z);
    }

}