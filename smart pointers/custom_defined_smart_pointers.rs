//---------------------------------------------
//      Custom Defined Smart Pointer
//          - almost always implemented using struct
//          - implement deref or drop traits
//---------------------------------------------

struct MySmartPointer{
    value: i32
}

impl MySmartPointer{
    fn new(x:i32)->MySmartPointer {
        MySmartPointer { value: x }
    }
}

// standard library implementation of Deref trait
// trait Deref{
//     type Target: name_of_type;
//     fn deref(&self) -> &self::Target
// }
    
use std::ops::Deref;

impl Deref for MySmartPointer{
    type Target = i32;
    fn deref(&self)-> &i32{
        &self.value
    }
}

// standard library implementation of Drop trait
/*
    pub trait Drop{
        fn drop(&mut self);
    }

*/
impl Drop for MySmartPointer{
    // this function is called when the MySmartPointer goes out of scope
    // gives back the memory allocated for that MySmartPointer
    fn drop(&mut self){
        println!("Dropping MySmartPointer object from memory {:?}",self.value);
    }
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

//    drop(sptr1);


}