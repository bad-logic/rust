use std::{thread, time::Duration};


fn main(){
    let job1 = thread::spawn(||{
        println!("--- Job 1 has started ---");
        println!("waiting for job 2 to complete");
        // thread::park();
        thread::park_timeout(Duration::from_secs(2));
        println!("--- Job 1 resumed ---");
        println!("--- Job 1 finished");
    });

    let job2 = thread::spawn(||{
        println!("--- Job2 started ---");
        println!("--- Job 2 finished");
    });

    job2.join().unwrap();
    println!("Job 2 is now complete");

    println!("Job 1 will now resume");
    job1.thread().unpark();
    job1.join().unwrap();
}