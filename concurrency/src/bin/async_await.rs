//------------------------------------------
//
//             Async Await Basics
//              tokio = {version="1.17", features=["full"]}
// 
//------------------------------------------

async fn printing() {
    println!("I am async function");
}

#[tokio::main]
async fn main(){

    // printing(); // no output since it is async and program exits before it could be executed
    let x = printing();
    println!("The async function has not being polled yet");
    // drop(x); // cancel the promise
    x.await;
}