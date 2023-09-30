use std::{path::Path, fs::{File, OpenOptions}, io::{Write, Read}};



fn write_to_file() -> std::io::Result<()>{
    let path_loc = r"test.txt";
    let path = Path::new(path_loc);
    
    // overwrites if file already exists
    let mut file = File::create(path)?;

    file.write(b"This is the new content of the file")?;
    file.write("\nlets use this line".as_bytes())?;


    // does not overwrite instead appends content to the file
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write("\nThis should be appended to the file".as_bytes())?;

    let some_vec = vec![1,2,3,4,5,6];
    let str_from_vec = some_vec.into_iter()
    .map(|a| format!("\n{}",a.to_string()))
    .collect::<String>();

    file.write(str_from_vec.as_bytes())?;
    Ok(())

}

fn read_file() -> std::io::Result<()>{
    let path_loc = r"test.txt";
    let path = Path::new(path_loc);

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("File contents:\n{}",contents);

    Ok(())
}

fn main(){
    write_to_file();
    read_file();
}