//------------------------------------------------------
// 
//          Threads Basics
// 
// 
// 
//------------------------------------------------------

use std::thread;
use std::time::Duration;

fn main(){
    println!("this will be printed");
    println!("this will also be printed");
    println!("the concurrency will start after this line");

    let t = thread::spawn(|| {
        println!("Hello 1 from the thread");
        println!("Hello 2 from the thread");
        println!("Hello 3 from the thread");
        println!("Hello 4 from the thread");
    });

    // halting main thread so that the above thread has chance of execution
    // because if the main thread completes its tasks it exits and does
    // not care if other threads has completed execution or not
    // thread::sleep(Duration::from_millis(1));

    // ensure that spawn thread runs to completion 
    t.join(); // blocks main thread until spawned thread completes its execution


    println!("Hello 1 from the main");
    println!("Hello 2 from the main");
}