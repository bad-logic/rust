//-----------------------------------------------------
// 
//          sharing states
//          Mutual Exclusive (Mutex)
// 
// 
//-----------------------------------------------------


use std::sync::Mutex;


fn main(){

    let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap(); // mutexGuard when goes out of scope, it releases the lock
    //     *num = 10;
    // }

    // println!("m= {:#?}",m);

    // let mut num1 = m.lock().unwrap();
    // let mut num2 = m.lock().unwrap();

    // *num1 = 10;
    // *num2 = 15; // deadlock

    // correct way
    let mut num1 = m.lock().unwrap();
    *num1 = 10;
    drop(num1);

    let mut num2 = m.lock().unwrap();
    *num2 = 15; 
    drop(num2);

    println!("m={:#?}",m);

}