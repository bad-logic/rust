//------------------------------------
// 
//      Sending Multiple Messages
//      Multiple Producers
//      Threads and Functions
// 
// 
// 
//------------------------------------


use std::thread;
use std::sync::mpsc;
use std::time::Duration;


fn main(){

    // Sending Multiple Messages
    let (tx,rx) = mpsc::channel();

    let t = thread::spawn(move || {
        let my_vec = vec![1,2,3,4,5];
        for i in my_vec {
            tx.send(i).unwrap();
        }
    });

    for received_val in rx{
        println!("Received[Multiple Messages]: {}", received_val);
    }

    // Multiple Producers
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let my_vec = vec![1,2,3,4,5];
        for i in my_vec {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    thread::spawn(move || {
        let my_vec = vec![6,7,8,9,10];
        for i in my_vec {
            tx_clone.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for i in rx{
        println!("Received[Multiple Producers]: {}",i);
    }


    // Threads and Functions
    let (tx,rx) = mpsc::channel();
    for i in 0..5{
        timer(i, tx.clone());
    }
    drop(tx);

    for i in rx{
        println!("Received[Threads and Functions]: {}",i);
    }

}

fn timer(d:i32,tx:mpsc::Sender<i32>){
    thread::spawn(move || {
        tx.send(d);
    });
}