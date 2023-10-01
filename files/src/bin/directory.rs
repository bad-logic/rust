//------------------------------------------------
//
//      Directory and path related functions
//
//------------------------------------------------

use std::{path::Path, env, fs::{self, File}};


fn main(){
    
    // FILE
    println!("creating a file {:?}", File::create(r"test.txt"));
    
    let path = Path::new(r"test.txt");
    println!("folder containing the file: {:?}", path.parent().unwrap());
    println!("name of the file is {:?}", path.file_stem().unwrap());
    println!("Extension of the file is {:?}", path.extension().unwrap());
    println!("isFile {:?}",path.is_file());

    let meta = path.metadata().unwrap();
    println!("type {:#?}",meta.file_type());
    println!("permissions {:#?}",meta.permissions());
    println!("length {:#?}",meta.len());
    println!("modified {:#?}", meta.modified());
    println!("created {:#?}", meta.created());

    // RENAME FILE
    println!("Renaming file {:?}", fs::rename(r"test.txt", r"tests.txt"));
    // REMOVE FILE
    println!("Removing a file {:?}", fs::remove_file(r"tests.txt"));

    let curr_path = env::current_dir().expect("can't access current directory");
    println!("current path: {:?}",curr_path);


    // DIRECTORY
    let path = Path::new(&curr_path);
    println!("isDirectory {:?}", path.is_dir());
    for files in path.read_dir().expect("read_dir call failed"){
        println!("{:?}", files.unwrap().path());
    }

    println!("Create a new directory: {:?}",fs::create_dir(r"test"));
    println!("Create a new directory and sub directory: {:?}",fs::create_dir_all(r"tests/test1/test2"));

    // REMOVE DIRECTORY
    println!("Remove created directory: {:?}", fs::remove_dir(r"test"));
    println!("Remove Everything from created directory: {:?}", fs::remove_dir_all(r"tests"));
}