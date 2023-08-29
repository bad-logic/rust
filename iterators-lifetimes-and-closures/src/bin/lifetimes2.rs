struct Person<'a>{
    name: &'a str,
    age: i32
}

fn main(){

    let first_name: &str = "Bob";
    let mut p1:Person = Person{
        name: &first_name,
        age: 40
    };

    {
        let last_name: String = String::from("Azam");
        p1.name = &last_name;
    }

    // lifetime of name and the struct should be same
    // thus below line will result in error
    // if we remove the below line code will compile just fine as the struct 
    // and name have same lifetime

    // println!("The name of the person is {} and his age is {}",p1.name, p1.age);

}