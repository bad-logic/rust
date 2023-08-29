pub mod file_1;
pub mod file_2;

fn print(status:&str){
    println!("{}",status);
}

pub fn print_status(){
    print("File: lib.rs")
}