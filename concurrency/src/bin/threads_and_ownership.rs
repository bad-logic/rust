

use std::thread;

fn main(){
    let mut thread_vec = vec![];

    for i in 0..10{
        thread_vec.push(thread::spawn(move|| {
            println!("Thread number {}",i);
        }));
    }

    for i in thread_vec{
        i.join();
    }


    let v = vec![1,2,3];
    let x =5;
    let handle = thread::spawn(move || {
        println!("vector: {:?}",v);
        println!("x: {}",x);
    });

    drop(x); // drop does not work with primitive data type
    println!("vector: {:?}",v);//  v has been moved to the thread, ownership changed so invalid
    println!("x: {}",x); // move action has different impact on primitive type
    handle.join();
}

