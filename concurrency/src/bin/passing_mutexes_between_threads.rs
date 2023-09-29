//------------------------------------------------
//
//              Passing Mutexes between threads
//
//------------------------------------------------


// use std::rc::Rc;
use std::sync::Arc; // atomic rc pointer thread safe
use std::sync::Mutex;
use std::thread;

// fn main(){
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10{
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles{
//         handle.join().unwrap();
//     }

//     println!("Result: {}",*counter.lock().unwrap());
// }

// cloning types that does not implement the clone trait using Arc Smart Pointer
struct MyString(String);

impl MyString {
    fn new(s:&str)-> MyString{
        MyString(s.to_string())
    }
}

fn main(){
    let mut threads = Vec::new();
    let name = Arc::new(MyString::new("Rust"));

    for i in 0..5 {
        let some_str = name.clone();// MyString does not implement clone still we can clone
        let t = thread::spawn(move || {
            println!("string: {}, count: {}", some_str.0,i);
        });
        threads.push(t);
    }

    for t in threads{
        t.join().unwrap()
    }
}
