//--------------------------------------------------
// 
//              Message Passing through channels
// 
//-------------------------------------------------

use std::thread;
use std::sync::mpsc;


fn main(){
    let (tx,rx) = mpsc::channel();

    thread::spawn(move|| {
        let val = String::from("Some data from sender");
        println!("value sending from the thread");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap(); // blocks main thread until some data is received
    println!("Received: {}",received);


    let (tx,rx) = mpsc::channel();

    let t = thread::spawn(move|| {
        let val = String::from("hello from the sender");
        tx.send(val).unwrap();
    });

    // t.join(); // blocks main thread

    let mut received_status = false;
    while received_status != true {
        match rx.try_recv() { // does not block the main thread
            Ok(received_value)=>{
                println!("Received: {}",received_value);
                received_status = true;
            },
            Err(_) => println!("I am doing some other stuff")
        }
    }
}